use std::fmt::Write as _;
use std::fs::File;
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use extsort::{ExternalSorter, Sortable};
use flate2::write::GzEncoder;
use flate2::Compression;
use structopt::StructOpt;

use tongrams::{util, GramsFileFormats, Vocabulary};

#[derive(StructOpt, Debug)]
#[structopt(name = "sort_grams", about = "A program to sort ngram file.")]
struct Opt {
    #[structopt(
        short = "f",
        long,
        default_value = "gzip",
        help = "Output file format from plain, gzip, deflate, and zlib."
    )]
    file_format: GramsFileFormats,

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
        let record_id = reader.read_u64::<LittleEndian>().ok()? as usize;
        Some(Self {
            mapped_ids,
            record_id,
        })
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let file_format = opt.file_format;
    let grams_filepath = opt.grams_filepath;
    let vocab_filepath = opt.vocab_filepath;
    let output_filepath = opt.output_filepath;

    println!("Loading the vocabulary: {:?}", vocab_filepath);
    let vocab = util::build_vocabulary_from_file(vocab_filepath, file_format)?;

    println!("Loading the records: {:?}", grams_filepath);
    let records = util::load_records_from_file(grams_filepath, file_format)?;
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

    let mut output_filename = output_filepath.into_os_string().into_string().unwrap();
    if let Some(ext) = util::get_format_extension(file_format) {
        write!(output_filename, ".{}", ext)?;
    }
    println!("Writing the index into {:?}", output_filename);

    let write_records = |mut writer: Box<dyn Write>| -> Result<()> {
        writer.write_fmt(format_args!("{}\n", num_grams))?;
        for mr in sorted_iter {
            let rec = &records[mr.record_id];
            writer.write_fmt(format_args!("{}\t{}\n", rec.gram(), rec.count()))?;
        }
        Ok(())
    };

    match file_format {
        GramsFileFormats::Plain => {
            let f = BufWriter::new(File::create(output_filename)?);
            write_records(Box::new(f))?
        }
        GramsFileFormats::Gzip => {
            let f = BufWriter::new(File::create(output_filename)?);
            write_records(Box::new(GzEncoder::new(f, Compression::default())))?;
        }
    };

    Ok(())
}
