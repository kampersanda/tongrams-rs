use std::io::{Read, Write};

use anyhow::Result;
use sucds::{EliasFano, EliasFanoBuilder, EliasFanoList};

use crate::trie_array::TrieArray;

#[derive(Default)]
pub struct EliasFanoTrieArray {
    token_ids: EliasFano,
    sampled_ids: EliasFano,
    count_ranks: EliasFanoList,
    pointers: EliasFano,
}

impl TrieArray for EliasFanoTrieArray {
    fn new(token_ids: Vec<usize>, count_ranks: Vec<usize>, pointers: Vec<usize>) -> Box<Self> {
        let (token_ids, sampled_ids) = Self::build_token_sequence(token_ids, &pointers);

        let mut pointer_efb =
            EliasFanoBuilder::new(pointers.last().unwrap() + 1, pointers.len()).unwrap();
        pointer_efb.append(&pointers).unwrap();

        Box::new(Self {
            token_ids,
            sampled_ids,
            count_ranks: EliasFanoList::from_slice(&count_ranks).unwrap(),
            pointers: EliasFano::new(pointer_efb, true),
        })
    }

    fn with_count_ranks(count_ranks: Vec<usize>) -> Box<Self> {
        Box::new(Self {
            token_ids: EliasFano::default(),
            sampled_ids: EliasFano::default(),
            count_ranks: EliasFanoList::from_slice(&count_ranks).unwrap(),
            pointers: EliasFano::default(),
        })
    }

    fn serialize_into<W>(&self, mut writer: W) -> Result<usize>
    where
        W: Write,
    {
        Ok(self.token_ids.serialize_into(&mut writer)?
            + self.sampled_ids.serialize_into(&mut writer)?
            + self.count_ranks.serialize_into(&mut writer)?
            + self.pointers.serialize_into(&mut writer)?)
    }

    fn deserialize_from<R>(mut reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let token_ids = EliasFano::deserialize_from(&mut reader)?;
        let sampled_ids = EliasFano::deserialize_from(&mut reader)?;
        let count_ranks = EliasFanoList::deserialize_from(&mut reader)?;
        let pointers = EliasFano::deserialize_from(&mut reader)?;
        Ok(Box::new(Self {
            token_ids,
            sampled_ids,
            count_ranks,
            pointers,
        }))
    }

    fn size_in_bytes(&self) -> usize {
        self.token_ids.size_in_bytes()
            + self.sampled_ids.size_in_bytes()
            + self.count_ranks.size_in_bytes()
            + self.pointers.size_in_bytes()
    }

    fn memory_statistics(&self) -> serde_json::Value {
        let token_ids = self.token_ids.size_in_bytes();
        let sampled_ids = self.sampled_ids.size_in_bytes();
        let count_ranks = self.count_ranks.size_in_bytes();
        let pointers = self.pointers.size_in_bytes();
        serde_json::json!({
            "token_ids": token_ids,
            "sampled_ids": sampled_ids,
            "count_ranks": count_ranks,
            "pointers": pointers,
        })
    }

    /// Gets the token id with a given index.
    fn token_id(&self, i: usize) -> usize {
        let pos = self.pointers.rank(i + 1) - 1;
        self.token_ids.select(i) - self.sampled_ids.select(pos)
    }

    fn count_rank(&self, i: usize) -> usize {
        self.count_ranks.get(i)
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
    ) -> (EliasFano, EliasFano) {
        assert_eq!(token_ids.len(), *pointers.last().unwrap());

        let mut sampled_id = 0;
        let mut sampled_ids = vec![0; pointers.len()];

        for i in 0..pointers.len() - 1 {
            sampled_ids[i] = sampled_id;
            let (b, e) = (pointers[i], pointers[i + 1]);
            debug_assert!(b <= e);
            for token_id in token_ids.iter_mut().take(e).skip(b) {
                *token_id += sampled_id;
            }
            if e != 0 {
                sampled_id = token_ids[e - 1];
            }
        }
        sampled_ids[pointers.len() - 1] = sampled_id;

        let mut token_efb = EliasFanoBuilder::new(sampled_id + 1, token_ids.len()).unwrap();
        token_efb.append(&token_ids).unwrap();

        let mut sampled_efb = EliasFanoBuilder::new(sampled_id + 1, sampled_ids.len()).unwrap();
        sampled_efb.append(&sampled_ids).unwrap();

        (
            EliasFano::new(token_efb, false),
            EliasFano::new(sampled_efb, false),
        )
    }
}
