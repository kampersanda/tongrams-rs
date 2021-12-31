use std::fs::File;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "stats", about = "A program to print memory statistics.")]
struct Opt {
    #[structopt(short = "i")]
    index_file: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let index_file = opt.index_file;

    eprintln!("Loading the index from {}...", &index_file);
    let reader = File::open(&index_file)?;
    let lm = EliasFanoTrieCountLm::deserialize_from(&reader)?;

    let mem_stats = lm.memory_statistics();
    println!("{}", mem_stats.to_string());

    Ok(())
}
