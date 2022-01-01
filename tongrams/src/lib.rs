//! # `tongrams`: Tons of *N*-grams
//!
//! `tongrams` is a crate to index and query large language models in compressed space, in which the data structures are presented in the following papers:
//!
//!  - Giulio Ermanno Pibiri and Rossano Venturini, [Efficient Data Structures for Massive N-Gram Datasets](https://doi.org/10.1145/3077136.3080798). In *Proceedings of the 40th ACM Conference on Research and Development in Information Retrieval (SIGIR 2017)*, pp. 615-624.
//!
//!  - Giulio Ermanno Pibiri and Rossano Venturini, [Handling Massive N-Gram Datasets Efficiently](https://doi.org/10.1145/3302913). *ACM Transactions on Information Systems (TOIS)*, 37.2 (2019): 1-41.
//!
//! This is a Rust port of [`tongrams`](https://github.com/jermp/tongrams) C++ library.
//!
//! ## What can do
//!
//!  - Store *N*-gram language models with frequency counts.
//!
//!  - Look up *N*-grams to get the frequency  counts.
//!
//! ## Features
//!
//!  - **Compressed language model.** `tongrams-rs` can store large *N*-gram language models in very compressed space. For example, the word *N*-gram datasets (*N*=1..5) in `test_data` are stored in only 2.6 bytes per gram.
//!   
//!  - **Time and memory efficiency.** `tongrams-rs` employs *Elias-Fano Trie*, which cleverly encodes a trie data structure consisting of *N*-grams through *Elias-Fano codes*, enabling fast lookups in compressed space.
//!   
//!  - **Pure Rust.** `tongrams-rs` is written only in Rust and can be easily pluged into your Rust codes.
//!
//! ## Input data format
//!
//! As with the original library, the *N*-gram counts files follow the [Google format](http://storage.googleapis.com/books/ngrams/books/datasetsv2.html).
//! For the details, please see [README](https://github.com/kampersanda/tongrams-rs/blob/main/README.md) of this repository.
//!
//! ## Examples
//!
//! The following code uses datasets in [`test_data`](https://github.com/kampersanda/tongrams-rs/tree/main/test_data) at the root of this repository.
//!
//! ```
//! use tongrams::EliasFanoTrieCountLm;
//!
//! // File names of N-grams.
//! let filenames = vec![
//!     "../test_data/1-grams.sorted.gz",
//!     "../test_data/2-grams.sorted.gz",
//!     "../test_data/3-grams.sorted.gz",
//! ];
//!
//! // Builds the language model from n-gram counts files.
//! let lm = EliasFanoTrieCountLm::from_gz_files(&filenames).unwrap();
//!
//! // Creates the instance for lookup.
//! let mut lookuper = lm.lookuper();
//!
//! // Gets the count of a query N-gram written in a space-separated string.
//! assert_eq!(lookuper.with_str("vector"), Some(182));
//! assert_eq!(lookuper.with_str("in order"), Some(47));
//! assert_eq!(lookuper.with_str("the same memory"), Some(8));
//! assert_eq!(lookuper.with_str("vector is array"), None);
//!
//! // Serializes the index into a writable stream.
//! let mut data = vec![];
//! lm.serialize_into(&mut data).unwrap();
//!
//! // Deserializes the index from a readable stream.
//! let other = EliasFanoTrieCountLm::deserialize_from(&data[..]).unwrap();
//! assert_eq!(lm.num_orders(), other.num_orders());
//! assert_eq!(lm.num_grams(), other.num_grams());
//! ```
pub mod gram;
pub mod loader;
pub mod parser;
pub mod record;
pub mod trie_count_lm;
pub mod util;
pub mod vocabulary;

mod mappers;
mod rank_array;
mod trie_array;

/// The maximum order of *N*-grams (i.e., `1 <= N <= 8`).
pub const MAX_ORDER: usize = 8;
/// The separator for tokens.
pub const TOKEN_SEPARATOR: u8 = b' ';
/// The separator for grams and count.
pub const GRAM_COUNT_SEPARATOR: u8 = b'\t';

pub use gram::Gram;
pub use record::Record;
pub use trie_count_lm::TrieCountLm;

pub use loader::GramsLoader;
pub use parser::GramsParser;
pub use vocabulary::Vocabulary;

use rank_array::{EliasFanoRankArray, SimpleRankArray};
use trie_array::{EliasFanoTrieArray, SimpleTrieArray};
use vocabulary::{DoubleArrayVocabulary, SimpleVocabulary};

/// Simple implementation of [`TrieCountLm`].
/// Note that this is for debug, and do NOT use it for storing massive datasets.
pub type SimpleTrieCountLm = TrieCountLm<SimpleTrieArray, SimpleVocabulary, SimpleRankArray>;

/// Elias-Fano Trie implementation of [`TrieCountLm`].
/// This configuration is similar to `ef_trie_PSEF_ranks_count_lm` in the original `tongrams`.
pub type EliasFanoTrieCountLm =
    TrieCountLm<EliasFanoTrieArray, DoubleArrayVocabulary, EliasFanoRankArray>;

fn handle_bincode_error(e: std::boxed::Box<bincode::ErrorKind>) -> anyhow::Error {
    anyhow::anyhow!("{:?}", e)
}
