use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::util;
use tongrams::vocabulary::Vocabulary;

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

    for (i, rec) in records.iter().enumerate() {
        let tokens = rec.gram().split_to_tokens();
        if let Some(ord) = order {
            assert_eq!(ord, tokens.len());
        } else {
            order = Some(tokens.len());
        }

        let mut mapped_ids = Vec::with_capacity(tokens.len());
        for token in tokens {
            mapped_ids.push(vocab.get(token).unwrap());
        }

        mapped_records.push((mapped_ids, i));
    }
    mapped_records.sort();

    println!("Writing the index into {:?}", output_filepath);
    let mut out_file = File::create(output_filepath)?;
    out_file.write_fmt(format_args!("{}\n", mapped_records.len()))?;
    for (_, i) in mapped_records {
        let rec = &records[i];
        out_file.write_fmt(format_args!("{}\t{}\n", rec.gram(), rec.count()))?;
    }

    Ok(())
}
