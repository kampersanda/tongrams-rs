use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "predict", about = "A program to build and write the index.")]
struct Opt {
    #[structopt(short = "i")]
    input_dir: PathBuf,

    #[structopt(short = "n")]
    order: usize,

    #[structopt(short = "o")]
    index_file: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let order = opt.order;
    let input_dir = opt.input_dir;
    let index_file = opt.index_file;

    let mut input_files = vec![];
    for i in 1..=order {
        let mut input_file = input_dir.clone();
        input_file.push(format!("{}-grams.sorted.gz", i));
        input_files.push(input_file);
    }
    println!("Input files: {:?}", input_files);

    println!("Counstructing the index...");
    let start = std::time::Instant::now();
    let lm = EliasFanoTrieCountLm::from_gz_files(&input_files)?;
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
