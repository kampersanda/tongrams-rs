use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "predict", about = "A program to build and write the index.")]
struct Opt {
    #[structopt(short = "i")]
    gram_files: Vec<PathBuf>,

    #[structopt(short = "o")]
    index_file: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let gram_files = opt.gram_files;
    let index_file = opt.index_file;

    println!("Counstructing the index...");
    let start = std::time::Instant::now();
    let lm = EliasFanoTrieCountLm::from_files(&gram_files)?;
    let duration = start.elapsed();
    println!("Elapsed time: {:.3} [sec]", duration.as_secs_f64());

    let num_grams = lm.num_grams();
    println!("{} grams are stored.", num_grams);

    println!("Writing the index into {}...", &index_file);
    let mut writer = File::create(&index_file)?;
    let mem = lm.serialize_into(&mut writer)?;
    println!(
        "Index size: {} bytes ({:.3} MiB)",
        mem,
        mem as f64 / (1024.0 * 1024.0)
    );

    println!("Bytes per gram: {:.3} bytes", mem as f64 / num_grams as f64,);

    Ok(())
}