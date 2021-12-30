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

    fn with_count_ranks(count_ranks: Vec<usize>) -> Box<Self> {
        Box::new(Self {
            token_ids: vec![],
            count_ranks,
            pointers: vec![],
        })
    }

    /// Gets the token id with a given index.
    fn token_id(&self, i: usize) -> usize {
        self.token_ids[i]
    }

    fn count_rank(&self, i: usize) -> usize {
        self.count_ranks[i]
    }

    fn range(&self, pos: usize) -> (usize, usize) {
        (self.pointers[pos], self.pointers[pos + 1])
    }

    fn find_token(&self, pos: usize, id: usize) -> Option<usize> {
        let (b, e) = self.range(pos);
        self.token_ids[b..e]
            .iter()
            .position(|&x| x == id)
            .map(|i| i + b)
    }

    fn num_tokens(&self) -> usize {
        self.token_ids.len()
    }

    fn num_pointers(&self) -> usize {
        self.pointers.len()
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
