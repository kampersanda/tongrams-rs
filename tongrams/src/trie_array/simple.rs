use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::handle_bincode_error;
use crate::trie_array::TrieArray;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleTrieArray {
    token_ids: Vec<usize>,
    pointers: Vec<usize>,
}

impl TrieArray for SimpleTrieArray {
    fn new(token_ids: Vec<usize>, pointers: Vec<usize>) -> Box<Self> {
        Box::new(Self {
            token_ids,
            pointers,
        })
    }

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        bincode::serialize_into(writer, self).map_err(handle_bincode_error)?;
        Ok(0)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let x: Self = bincode::deserialize_from(reader).map_err(handle_bincode_error)?;
        Ok(Box::new(x))
    }

    fn size_in_bytes(&self) -> usize {
        0
    }

    fn memory_statistics(&self) -> serde_json::Value {
        serde_json::json!({})
    }

    /// Gets the token id with a given index.
    fn token_id(&self, i: usize) -> usize {
        self.token_ids[i]
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
}
