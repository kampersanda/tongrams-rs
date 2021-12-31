pub mod gram;
pub mod loader;
pub mod mappers;
pub mod parser;
pub mod rank_array;
pub mod record;
pub mod trie_array;
pub mod trie_count_lm;
pub mod vocabulary;

pub use gram::Gram;
pub use rank_array::{EliasFanoRankArray, SimpleRankArray};
pub use record::Record;
pub use trie_array::{EliasFanoTrieArray, SimpleTrieArray};
pub use trie_count_lm::TrieCountLm;
pub use vocabulary::{DoubleArrayVocabulary, SimpleVocabulary};

pub const MAX_ORDER: usize = 8;
pub const GRAM_SEPARATOR: u8 = b' ';
pub const GRAM_COUNT_SEPARATOR: u8 = b'\t';

pub type SimpleTrieCountLm = TrieCountLm<SimpleTrieArray, SimpleVocabulary, SimpleRankArray>;
pub type EliasFanoTrieCountLm =
    TrieCountLm<EliasFanoTrieArray, DoubleArrayVocabulary, EliasFanoRankArray>;

pub fn handle_bincode_error(e: std::boxed::Box<bincode::ErrorKind>) -> anyhow::Error {
    anyhow::anyhow!("{:?}", e)
}
