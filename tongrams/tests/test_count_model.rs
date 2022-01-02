use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

use tongrams::loader::{GramsGzFileLoader, GramsLoader};
use tongrams::EliasFanoTrieCountLm;

const TEST_FILENAMES: [&str; 5] = [
    "../test_data/1-grams.sorted.gz",
    "../test_data/2-grams.sorted.gz",
    "../test_data/3-grams.sorted.gz",
    "../test_data/4-grams.sorted.gz",
    "../test_data/5-grams.sorted.gz",
];

const NOEXIST_FILENAME: &str = "../test_data/queries.noexist.5K.txt";

const NUM_GRAMS: [usize; 5] = [8761, 38900, 61516, 70186, 73187];

fn load_noexist_queries() -> Vec<String> {
    let file = File::open(NOEXIST_FILENAME).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

#[test]
fn test_parser() {
    for (&filename, &num_grams) in TEST_FILENAMES.iter().zip(NUM_GRAMS.iter()) {
        let loader = GramsGzFileLoader::new(PathBuf::from_str(filename).unwrap());
        let parser = loader.parser().unwrap();
        assert_eq!(parser.num_grams(), num_grams);
    }
}

#[test]
fn test_lookup() {
    let lm = EliasFanoTrieCountLm::from_gz_files(&TEST_FILENAMES).unwrap();
    assert_eq!(lm.num_orders(), 5);

    let mut lookuper = lm.lookuper();
    for filename in TEST_FILENAMES {
        let loader = GramsGzFileLoader::new(filename);
        let parser = loader.parser().unwrap();
        for rec in parser {
            let rec = rec.unwrap();
            assert_eq!(lookuper.with_gram(rec.gram()), Some(rec.count()));
        }
    }
}

#[test]
fn test_noexist_lookup() {
    let lm = EliasFanoTrieCountLm::from_gz_files(&TEST_FILENAMES).unwrap();
    assert_eq!(lm.num_orders(), 5);

    let mut lookuper = lm.lookuper();
    let queries = load_noexist_queries();
    for query in &queries {
        assert_eq!(lookuper.with_str(query), None);
    }
}

#[test]
fn test_serialization() {
    let lm = EliasFanoTrieCountLm::from_gz_files(&TEST_FILENAMES).unwrap();

    let mut data = vec![];
    lm.serialize_into(&mut data).unwrap();

    let other = EliasFanoTrieCountLm::deserialize_from(&data[..]).unwrap();
    assert_eq!(lm.num_orders(), other.num_orders());
    assert_eq!(lm.num_grams(), other.num_grams());
}
