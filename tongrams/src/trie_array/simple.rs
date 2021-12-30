use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::handle_bincode_error;
use crate::trie_array::TrieArray;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleTrieArray {
    token_ids: Vec<usize>,
    count_ranks: Vec<usize>,
    pointers: Vec<usize>,
}

impl TrieArray for SimpleTrieArray {
    fn new(token_ids: Vec<usize>, count_ranks: Vec<usize>, pointers: Vec<usize>) -> Box<Self> {
        Box::new(Self {
            token_ids,
            count_ranks,
            pointers,
        })
    }

    fn range(&self, pos: usize) -> (usize, usize) {
        (self.pointers[pos], self.pointers[pos + 1])
    }

    /// Gets the token id with a given index.
    fn token_id(&self, pos: usize) -> usize {
        self.token_ids[pos]
    }

    fn count_rank(&self, pos: usize) -> usize {
        self.count_ranks[pos]
    }

    /// Searches for an element within a given range, returning its index.
    fn position(&self, rng: (usize, usize), id: usize) -> Option<usize> {
        self.token_ids[rng.0..rng.1]
            .iter()
            .position(|&x| x == id)
            .map(|i| i + rng.0)
    }

    fn serialize_into<W>(&self, writer: W) -> Result<()>
    where
        W: Write,
    {
        bincode::serialize_into(writer, self).map_err(handle_bincode_error)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let x: Self = bincode::deserialize_from(reader).map_err(handle_bincode_error)?;
        Ok(Box::new(x))
    }
}
