use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "stats", about = "A program to print memory statistics.")]
struct Opt {
    #[structopt(short = "i")]
    index_filepath: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let index_filepath = opt.index_filepath;

    eprintln!("Loading the index from {:?}...", &index_filepath);
    let mut reader = BufReader::new(File::open(&index_filepath)?);
    let lm = EliasFanoTrieCountLm::deserialize_from(&mut reader)?;

    let mem_stats = lm.memory_statistics();
    println!("{}", mem_stats);

    Ok(())
}
