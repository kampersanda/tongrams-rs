use std::io::{Read, Write};

use anyhow::Result;

use crate::trie_array::TrieArray;

#[derive(Default)]
pub struct EliasFanoTrieArray {
    token_ids: sucds::EliasFano,
    sampled_ids: sucds::EliasFano,
    pointers: sucds::EliasFano,
}

impl TrieArray for EliasFanoTrieArray {
    fn new(token_ids: Vec<usize>, pointers: Vec<usize>) -> Box<Self> {
        if token_ids.is_empty() {
            return Box::new(Self::default());
        }

        let (token_ids, sampled_ids) = Self::build_token_sequence(token_ids, &pointers);
        let pointers = Self::build_pointers(pointers);
        Box::new(Self {
            token_ids,
            sampled_ids,
            pointers,
        })
    }

    fn serialize_into<W>(&self, mut writer: W) -> Result<usize>
    where
        W: Write,
    {
        Ok(self.token_ids.serialize_into(&mut writer)?
            + self.sampled_ids.serialize_into(&mut writer)?
            + self.pointers.serialize_into(&mut writer)?)
    }

    fn deserialize_from<R>(mut reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let token_ids = sucds::EliasFano::deserialize_from(&mut reader)?;
        let sampled_ids = sucds::EliasFano::deserialize_from(&mut reader)?;
        let pointers = sucds::EliasFano::deserialize_from(&mut reader)?;
        Ok(Box::new(Self {
            token_ids,
            sampled_ids,
            pointers,
        }))
    }

    fn size_in_bytes(&self) -> usize {
        self.token_ids.size_in_bytes()
            + self.sampled_ids.size_in_bytes()
            + self.pointers.size_in_bytes()
    }

    fn memory_statistics(&self) -> serde_json::Value {
        let token_ids = self.token_ids.size_in_bytes();
        let sampled_ids = self.sampled_ids.size_in_bytes();
        let pointers = self.pointers.size_in_bytes();
        serde_json::json!({
            "token_ids": token_ids,
            "sampled_ids": sampled_ids,
            "pointers": pointers,
        })
    }

    /// Gets the token id with a given index.
    fn token_id(&self, i: usize) -> usize {
        let pos = self.pointers.rank(i + 1) - 1;
        self.token_ids.select(i) - self.sampled_ids.select(pos)
    }

    fn range(&self, pos: usize) -> (usize, usize) {
        (self.pointers.select(pos), self.pointers.select(pos + 1))
    }

    /// Searches for an element within a given range, returning its index.
    fn find_token(&self, pos: usize, id: usize) -> Option<usize> {
        let (b, e) = self.range(pos);
        let sampled_id = self.sampled_ids.select(pos);

        // TODO: Use iterator
        for i in b..e {
            let token_id = self.token_ids.select(i) - sampled_id;
            if token_id == id {
                return Some(i);
            }
        }
        None
    }

    fn num_tokens(&self) -> usize {
        self.token_ids.len()
    }

    fn num_pointers(&self) -> usize {
        self.pointers.len()
    }
}

impl EliasFanoTrieArray {
    fn build_token_sequence(
        mut token_ids: Vec<usize>,
        pointers: &[usize],
    ) -> (sucds::EliasFano, sucds::EliasFano) {
        assert_eq!(token_ids.len(), *pointers.last().unwrap());

        let mut sampled_id = 0;
        let mut sampled_ids = vec![0; pointers.len()];

        for i in 0..pointers.len() - 1 {
            let (b, e) = (pointers[i], pointers[i + 1]);
            debug_assert!(b <= e);

            sampled_ids[i] = sampled_id;
            for token_id in token_ids.iter_mut().take(e).skip(b) {
                *token_id += sampled_id;
            }
            if e != 0 {
                sampled_id = token_ids[e - 1];
            }
        }
        sampled_ids[pointers.len() - 1] = sampled_id;

        let mut token_efb = sucds::EliasFanoBuilder::new(sampled_id + 1, token_ids.len()).unwrap();
        token_efb.append(&token_ids).unwrap();

        let mut sampled_efb =
            sucds::EliasFanoBuilder::new(sampled_id + 1, sampled_ids.len()).unwrap();
        sampled_efb.append(&sampled_ids).unwrap();

        (
            sucds::EliasFano::new(token_efb, false),
            sucds::EliasFano::new(sampled_efb, false),
        )
    }

    fn build_pointers(pointers: Vec<usize>) -> sucds::EliasFano {
        let mut pointer_efb =
            sucds::EliasFanoBuilder::new(pointers.last().unwrap() + 1, pointers.len()).unwrap();
        pointer_efb.append(&pointers).unwrap();
        sucds::EliasFano::new(pointer_efb, true)
    }
}
