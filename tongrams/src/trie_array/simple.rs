use std::io::{Read, Write};

use anyhow::Result;

use crate::trie_array::TrieArray;

#[derive(Default, Debug)]
pub struct SimpleTrieArray {
    token_ids: Vec<usize>,
    pointers: Vec<usize>,
}

impl TrieArray for SimpleTrieArray {
    fn build(token_ids: Vec<usize>, pointers: Vec<usize>) -> Box<Self> {
        Box::new(Self {
            token_ids,
            pointers,
        })
    }

    fn serialize_into<W>(&self, mut writer: W) -> Result<usize>
    where
        W: Write,
    {
        Ok(
            sucds::util::int_vector::serialize_into(&self.token_ids, &mut writer)?
                + sucds::util::int_vector::serialize_into(&self.pointers, &mut writer)?,
        )
    }

    fn deserialize_from<R>(mut reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let token_ids = sucds::util::int_vector::deserialize_from(&mut reader)?;
        let pointers = sucds::util::int_vector::deserialize_from(&mut reader)?;
        Ok(Box::new(Self {
            token_ids,
            pointers,
        }))
    }

    fn size_in_bytes(&self) -> usize {
        sucds::util::int_vector::size_in_bytes(&self.token_ids)
            + sucds::util::int_vector::size_in_bytes(&self.pointers)
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
