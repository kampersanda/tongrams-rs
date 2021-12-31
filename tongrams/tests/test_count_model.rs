use std::path::PathBuf;
use std::str::FromStr;

use tongrams::loader::{GramsFileLoader, GramsLoader};
use tongrams::EliasFanoTrieCountLm;

const TEST_FILENAMES: [&str; 5] = [
    "../test_data/1-grams.sorted",
    "../test_data/2-grams.sorted",
    "../test_data/3-grams.sorted",
    "../test_data/4-grams.sorted",
    "../test_data/5-grams.sorted",
];

const NUM_GRAMS: [usize; 5] = [8761, 38900, 61516, 70186, 73187];

#[test]
fn test_parser() {
    for (&filename, &num_grams) in TEST_FILENAMES.iter().zip(NUM_GRAMS.iter()) {
        let loader = GramsFileLoader::new(PathBuf::from_str(filename).unwrap());
        let parser = loader.parser().unwrap();
        assert_eq!(parser.num_grams(), num_grams);
    }
}

#[test]
fn test_lookup() {
    let filepaths: Vec<PathBuf> = TEST_FILENAMES
        .iter()
        .map(|f| PathBuf::from_str(f).unwrap())
        .collect();
    let lm = EliasFanoTrieCountLm::from_files(&filepaths).unwrap();
    assert_eq!(lm.num_orders(), 5);

    let mut lookuper = lm.lookuper();
    for filepath in filepaths {
        let loader = GramsFileLoader::new(filepath);
        let parser = loader.parser().unwrap();
        for rec in parser {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }
    }

    // TODO: Add not-found test
}

#[test]
fn test_serialization() {
    let filepaths: Vec<PathBuf> = TEST_FILENAMES
        .iter()
        .map(|f| PathBuf::from_str(f).unwrap())
        .collect();
    let lm = EliasFanoTrieCountLm::from_files(&filepaths).unwrap();

    let mut data = vec![];
    lm.serialize_into(&mut data).unwrap();

    let other = EliasFanoTrieCountLm::deserialize_from(&data[..]).unwrap();
    assert_eq!(lm.num_orders(), other.num_orders());
    assert_eq!(lm.num_grams(), other.num_grams());
}
