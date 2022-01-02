use std::fs::File;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use rand::prelude::*;
use structopt::StructOpt;

use tongrams::{util, EliasFanoTrieCountLm, GramsFileFormats};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "make_noexist_queries",
    about = "A program to make queries not contained in the index."
)]
struct Opt {
    #[structopt(
        short = "f",
        long,
        default_value = "gzip",
        help = "Input file format from plain, gzip, deflate, and zlib."
    )]
    file_format: GramsFileFormats,

    #[structopt(short = "i")]
    index_filepath: PathBuf,

    #[structopt(short = "v")]
    vocab_filepath: PathBuf,

    #[structopt(short = "n")]
    num_queirs: usize,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let file_format = opt.file_format;
    let index_filepath = opt.index_filepath;
    let vocab_filepath = opt.vocab_filepath;
    let num_queirs = opt.num_queirs;

    if num_queirs == 0 {
        return Err(anyhow!("num_queirs must be more than zero."));
    }

    let lm = {
        let reader = File::open(&index_filepath)?;
        EliasFanoTrieCountLm::deserialize_from(&reader)?
    };
    let records = util::load_records_from_file(vocab_filepath, file_format)?;

    let max_order = lm.num_orders();
    let mut lookuper = lm.lookuper();

    let mut rng = rand::thread_rng();
    let mut count = 0;

    loop {
        let order = rng.gen_range(1..=max_order);
        let mut tokens = Vec::with_capacity(order);
        for _ in 1..=order {
            let i = rng.gen_range(0..records.len());
            tokens.push(String::from_utf8(records[i].gram().to_vec())?);
        }
        let query = tokens.join(" ");
        if lookuper.with_str(&query).is_none() {
            println!("{}", query);
            count += 1;
            if count == num_queirs {
                break;
            }
        }
    }

    Ok(())
}
