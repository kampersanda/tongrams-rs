use std::fs::File;
use std::io::BufReader;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "lookup", about = "A demo program to lookup ngrams.")]
struct Opt {
    #[structopt(short = "i")]
    index_filepath: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let index_filepath = opt.index_filepath;

    println!("Loading the index from {:?}...", &index_filepath);
    let mut reader = BufReader::new(File::open(&index_filepath)?);
    let lm = EliasFanoTrieCountLm::deserialize_from(&mut reader)?;
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

        lookuper.with_str(query).map_or_else(
            || {
                println!("Not found");
            },
            |count| {
                println!("count = {}", count);
            },
        )
    }

    println!("Good bye!");
    Ok(())
}
