use tongrams::EliasFanoTrieCountLm;

fn main() {
    // File names of N-grams.
    let filenames = vec![
        "../test_data/1-grams.sorted.gz",
        "../test_data/2-grams.sorted.gz",
        "../test_data/3-grams.sorted.gz",
    ];

    // Builds the language model from n-gram counts files.
    let lm = EliasFanoTrieCountLm::from_gz_files(&filenames).unwrap();

    // Creates the instance for lookup.
    let mut lookuper = lm.lookuper();

    // Gets the count of a query N-gram written in a space-separated string.
    assert_eq!(lookuper.with_str("vector"), Some(182));
    assert_eq!(lookuper.with_str("in order"), Some(47));
    assert_eq!(lookuper.with_str("the same memory"), Some(8));
    assert_eq!(lookuper.with_str("vector is array"), None);

    // Serializes the index into a writable stream.
    let mut data = vec![];
    lm.serialize_into(&mut data).unwrap();

    // Deserializes the index from a readable stream.
    let other = EliasFanoTrieCountLm::deserialize_from(&data[..]).unwrap();
    assert_eq!(lm.num_orders(), other.num_orders());
    assert_eq!(lm.num_grams(), other.num_grams());
}
