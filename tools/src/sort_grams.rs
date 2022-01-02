use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use extsort::{ExternalSorter, Sortable};
use structopt::StructOpt;

use tongrams::util;
use tongrams::Vocabulary;

#[derive(StructOpt, Debug)]
#[structopt(name = "sort_grams", about = "A program to sort ngram file.")]
struct Opt {
    #[structopt(short = "i")]
    grams_filepath: PathBuf,

    #[structopt(short = "v")]
    vocab_filepath: PathBuf,

    #[structopt(short = "o")]
    output_filepath: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct MappedRecord {
    mapped_ids: Vec<usize>,
    record_id: usize,
}

impl Sortable for MappedRecord {
    fn encode<W: Write>(&self, writer: &mut W) {
        writer.write_u8(self.mapped_ids.len() as u8).unwrap();
        for &x in &self.mapped_ids {
            writer.write_u64::<LittleEndian>(x as u64).unwrap();
        }
        writer
            .write_u64::<LittleEndian>(self.record_id as u64)
            .unwrap();
    }

    fn decode<R: Read>(reader: &mut R) -> Option<Self> {
        let len = reader.read_u8().ok()? as usize;
        let mut mapped_ids = Vec::with_capacity(len);
        for _ in 0..len {
            mapped_ids.push(reader.read_u64::<LittleEndian>().ok()? as usize);
        }
        // let mapped_ids = sucds::util::int_vector::deserialize_from(reader).ok()?;
        let record_id = reader.read_u64::<LittleEndian>().ok()? as usize;
        Some(Self {
            mapped_ids,
            record_id,
        })
    }
}

// TODO: Make space-efficient with secondary memory

fn main() -> Result<()> {
    println!("WARNING: The current implementation will use a lot of memory.");

    let opt = Opt::from_args();
    let grams_filepath = opt.grams_filepath;
    let vocab_filepath = opt.vocab_filepath;
    let output_filepath = opt.output_filepath;

    println!("Loading the vocabulary: {:?}", vocab_filepath);
    let vocab = util::build_vocabulary_from_gz(vocab_filepath)?;

    println!("Loading the records: {:?}", grams_filepath);
    let records = util::load_records_from_gz(grams_filepath)?;
    let num_grams = records.len();

    println!("Sorting the records");
    let mut mapped_records = Vec::with_capacity(num_grams);
    let mut order: Option<usize> = None;

    for (record_id, rec) in records.iter().enumerate() {
        let tokens = rec.gram().split_to_tokens();
        order.map_or_else(
            || {
                order = Some(tokens.len());
            },
            |ord| {
                assert_eq!(ord, tokens.len());
            },
        );

        let mut mapped_ids = Vec::with_capacity(tokens.len());
        for token in tokens {
            mapped_ids.push(vocab.get(token).unwrap());
        }

        mapped_records.push(MappedRecord {
            mapped_ids,
            record_id,
        });
    }

    let sorter = ExternalSorter::new();
    let sorted_iter = sorter.sort(mapped_records.into_iter())?;

    println!("Writing the index into {:?}", output_filepath);
    let mut out_file = File::create(output_filepath)?;
    out_file.write_fmt(format_args!("{}\n", num_grams))?;
    for mr in sorted_iter {
        let rec = &records[mr.record_id];
        out_file.write_fmt(format_args!("{}\t{}\n", rec.gram(), rec.count()))?;
    }

    Ok(())
}
