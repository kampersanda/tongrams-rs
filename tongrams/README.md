# `tongrams`: Tons of *N*-grams

`tongrams` is a crate to index and query large language models in compressed space, in which the data structures are presented in the following papers:

 - Giulio Ermanno Pibiri and Rossano Venturini, [Efficient Data Structures for Massive N-Gram Datasets](https://doi.org/10.1145/3077136.3080798). In *Proceedings of the 40th ACM Conference on Research and Development in Information Retrieval (SIGIR 2017)*, pp. 615-624.

 - Giulio Ermanno Pibiri and Rossano Venturini, [Handling Massive N-Gram Datasets Efficiently](https://doi.org/10.1145/3302913). *ACM Transactions on Information Systems (TOIS)*, 37.2 (2019): 1-41.

This is a Rust port of [`tongrams`](https://github.com/jermp/tongrams) C++ library.

## What can do

 - Store *N*-gram language models with frequency counts.

 - Look up *N*-grams to get the frequency  counts.

## Features

 - **Compressed language model.** `tongrams-rs` can store large *N*-gram language models in very compressed space. For example, the word *N*-gram datasets (*N*=1..5) in `test_data` are stored in only 2.6 bytes per gram.
  
 - **Time and memory efficiency.** `tongrams-rs` employs *Elias-Fano Trie*, which cleverly encodes a trie data structure consisting of *N*-grams through *Elias-Fano codes*, enabling fast lookups in compressed space.
  
 - **Pure Rust.** `tongrams-rs` is written only in Rust and can be easily pluged into your Rust codes.

## Installation

To use `tongrams`, depend on it in your Cargo manifest:

```toml
# Cargo.toml

[dependencies]
tongrams = "0.1"
```

## Input data format

The file format of *N*-gram counts files is the same as that used in [`tongrams`](https://github.com/jermp/tongrams), a modified [Google format](http://storage.googleapis.com/books/ngrams/books/datasetsv2.html), where

 - one separate file for each distinct value of *N* (order) lists one gram per row,
 - each header row `<number_of_grams>` indicates the number of *N*-grams in the file,
 - tokens in a gram `<gram>` is sparated by a space (e.g., `the same time`), and
 - a gram `<gram>` and the count `<count>` is sparated by a horizontal tab.

```
<number_of_grams>
<gram1><TAB><count1>
<gram2><TAB><count2>
<gram3><TAB><count3>
...
```

## Examples

The following code uses datasets in [`test_data`](https://github.com/kampersanda/tongrams-rs/tree/main/test_data) at the root of this repository.

```rust
use tongrams::EliasFanoTrieCountLm;

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
```

## Licensing

This library is free software provided under MIT.
