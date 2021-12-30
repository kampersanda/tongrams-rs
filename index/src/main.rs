use std::fs::File;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "predict", about = "A program to build and write the index.")]
struct Opt {
    #[structopt(short = "i")]
    gram_files: Vec<String>,

    #[structopt(short = "o")]
    index_file: String,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let gram_files = opt.gram_files;

    let lm = EliasFanoTrieCountLm::from_files(gram_files).unwrap();

    let mut writer = File::create(opt.index_file)?;
    lm.serialize_into(&mut writer).unwrap();

    Ok(())
}
