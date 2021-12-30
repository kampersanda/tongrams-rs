use std::fs::File;
use std::io::{stdin, stdout, Write};

use anyhow::Result;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "lookup", about = "A demo program to lookup ngrams.")]
struct Opt {
    #[structopt(short = "i")]
    index_file: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let index_file = opt.index_file;

    println!("Loading the index from {}...", &index_file);
    let reader = File::open(&index_file)?;
    let lm = EliasFanoTrieCountLm::deserialize_from(&reader)?;
    let mut lookuper = lm.lookuper();

    println!("Performing the lookup...");
    let mut buf = String::new();
    loop {
        print!("> ");
        stdout().flush()?;

        buf.clear();
        stdin().read_line(&mut buf)?;

        let query = buf.trim();
        if query.is_empty() {
            break;
        }

        if let Some(count) = lookuper.with_str(&query) {
            println!("count = {}", count);
        } else {
            println!("Not found");
        }
    }

    println!("Thanks!");
    Ok(())
}
