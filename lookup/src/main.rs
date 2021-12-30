use std::fs::File;
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;

use tongrams::EliasFanoTrieCountLm;

#[derive(StructOpt, Debug)]
#[structopt(name = "lookup", about = "A demo program to lookup ngrams.")]
struct Opt {
    #[structopt(short = "i")]
    index_file: String,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let reader = File::open(opt.index_file)?;

    let lm = EliasFanoTrieCountLm::deserialize_from(&reader).unwrap();
    let mut lookuper = lm.lookuper();

    let mut buf = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

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
