pub mod gram;
pub mod loader;
pub mod mappers;
pub mod parser;
pub mod trie_array;
pub mod trie_count_lm;
pub mod vocabulary;

pub use gram::Gram;
pub use trie_array::{EliasFanoTrieArray, SimpleTrieArray};
pub use trie_count_lm::TrieCountLm;
pub use vocabulary::{DoubleArrayVocabulary, SimpleVocabulary};

pub const MAX_ORDER: usize = 8;
pub const GRAM_SEPARATOR: u8 = b' ';
pub const GRAM_COUNT_SEPARATOR: u8 = b'\t';

pub type SimpleTrieCountLm = TrieCountLm<SimpleTrieArray, SimpleVocabulary>;
pub type EliasFanoTrieCountLm = TrieCountLm<EliasFanoTrieArray, DoubleArrayVocabulary>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    gram: String, // TODO: Store as a byte slice to another buffer
    count: usize,
}

impl Record {
    pub fn new(gram: String, count: usize) -> Self {
        Self { gram, count }
    }

    pub fn gram<'a>(&'a self) -> Gram<'a> {
        Gram::new(self.gram.as_bytes())
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub fn handle_bincode_error(e: std::boxed::Box<bincode::ErrorKind>) -> anyhow::Error {
    anyhow::anyhow!("{:?}", e)
}
