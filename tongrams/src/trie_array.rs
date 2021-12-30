pub mod builder;
pub mod ef;
pub mod simple;

use std::io::{Read, Write};

use anyhow::Result;

pub use crate::trie_array::builder::TrieArrayBuilder;
pub use crate::trie_array::ef::EliasFanoTrieArray;
pub use crate::trie_array::simple::SimpleTrieArray;

pub trait TrieArray {
    fn new(token_ids: Vec<usize>, count_ranks: Vec<usize>, pointers: Vec<usize>) -> Box<Self>;

    fn with_count_ranks(count_ranks: Vec<usize>) -> Box<Self>;

    fn range(&self, pos: usize) -> (usize, usize);

    fn token_id(&self, pos: usize) -> usize;

    fn count_rank(&self, pos: usize) -> usize;

    fn position(&self, rng: (usize, usize), id: usize) -> Option<usize>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<()>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;
}
