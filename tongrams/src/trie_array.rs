pub mod builder;

use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::grams_sequence::SimpleGramsSequence;
use crate::handle_bincode_error;

pub use crate::trie_array::builder::TrieLayerBuilder;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleTrieArray {
    token_ids: SimpleGramsSequence,
    count_ranks: Vec<usize>,
    pointers: Vec<usize>,
}

impl SimpleTrieArray {
    pub fn range(&self, pos: usize) -> (usize, usize) {
        (self.pointers[pos], self.pointers[pos + 1])
    }

    pub fn token_id(&self, pos: usize) -> usize {
        self.token_ids.get(pos)
    }

    pub fn count_rank(&self, pos: usize) -> usize {
        self.count_ranks[pos]
    }

    pub fn position(&self, rng: (usize, usize), id: usize) -> Option<usize> {
        self.token_ids.position(rng, id)
    }

    pub fn serialize_into<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        bincode::serialize_into(writer, self).map_err(handle_bincode_error)
    }

    pub fn deserialize_from<R>(reader: R) -> Result<Self>
    where
        R: Read,
    {
        bincode::deserialize_from(reader).map_err(handle_bincode_error)
    }
}
