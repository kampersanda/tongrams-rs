use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::{util, EliasFanoTrieCountLm, GramsFileFormats};

#[derive(StructOpt, Debug)]
#[structopt(name = "index", about = "A program to build and write the index.")]
struct Opt {
    #[structopt(
        short = "f",
        long,
        default_value = "gzip",
        help = "Input file format from plain, gzip, deflate, and zlib."
    )]
    file_format: GramsFileFormats,

    #[structopt(short = "i", long)]
    grams_dirpath: PathBuf,

    #[structopt(short = "n", long)]
    order: usize,

    #[structopt(short = "o", long)]
    index_filepath: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let order = opt.order;
    let file_format = opt.file_format;
    let grams_dirpath = opt.grams_dirpath;
    let index_filepath = opt.index_filepath;

    let mut grams_filepaths = vec![];
    for i in 1..=order {
        let mut grams_filepath = grams_dirpath.clone();
        grams_filepath.push(format!("{}-grams.sorted", i));
        let mut grams_filename = grams_filepath.into_os_string().into_string().unwrap();
        if let Some(ext) = util::get_format_extension(file_format) {
            grams_filename.push_str(&format!(".{}", ext));
        }
        grams_filepaths.push(grams_filename);
    }
    println!("Input files: {:?}", grams_filepaths);

    println!("Counstructing the index...");
    let start = std::time::Instant::now();
    let lm = EliasFanoTrieCountLm::from_files(&grams_filepaths, file_format)?;
    let duration = start.elapsed();
    println!("Elapsed time: {:.3} [sec]", duration.as_secs_f64());

    let num_grams = lm.num_grams();
    println!("{} grams are stored.", num_grams);

    println!("Writing the index into {:?}...", &index_filepath);
    let mut writer = File::create(&index_filepath)?;
    let mem = lm.serialize_into(&mut writer)?;
    println!(
        "Index size: {} bytes ({:.3} MiB)",
        mem,
        mem as f64 / (1024.0 * 1024.0)
    );

    println!("Bytes per gram: {:.3} bytes", mem as f64 / num_grams as f64,);

    Ok(())
}
