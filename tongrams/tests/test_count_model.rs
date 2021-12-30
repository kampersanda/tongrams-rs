use tongrams::loader::{GramsFileLoader, GramsLoader};
use tongrams::TrieCountLm;

const TEST_FILENAMES: [&str; 5] = [
    "test_data/1-grams.sorted",
    "test_data/2-grams.sorted",
    "test_data/3-grams.sorted",
    "test_data/4-grams.sorted",
    "test_data/5-grams.sorted",
];

const NUM_GRAMS: [usize; 5] = [8761, 38900, 61516, 70186, 73187];

#[test]
fn test_parser() {
    for (filename, &num_grams) in TEST_FILENAMES.iter().zip(NUM_GRAMS.iter()) {
        let loader = GramsFileLoader::new(filename.to_string());
        let parser = loader.parser().unwrap();
        assert_eq!(parser.num_grams(), num_grams);
    }
}

#[test]
fn test_trie_count_lm() {
    let filenames = TEST_FILENAMES.iter().map(|f| f.to_string()).collect();
    let lm = TrieCountLm::from_files(filenames).unwrap();
    assert_eq!(lm.max_order(), 4);

    for filename in TEST_FILENAMES {
        let loader = GramsFileLoader::new(filename.to_string());
        let parser = loader.parser().unwrap();
        for rec in parser {
            let rec = rec.unwrap();
            assert_eq!(lm.lookup(rec.gram()), Some(rec.count()));
        }
    }

    // TODO: Add not-found test
}
