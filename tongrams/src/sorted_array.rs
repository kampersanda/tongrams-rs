use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::grams_sequence::SimpleGramsSequence;
use crate::handle_bincode_error;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleSortedArray {
    token_ids: SimpleGramsSequence,
    count_ranks: Vec<usize>,
    pointers: Vec<usize>,
}

impl SimpleSortedArray {
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

#[derive(Default)]
pub struct SortedArrayBuilder {
    token_ids: Vec<usize>,
    count_ranks: Vec<usize>,
}

impl SortedArrayBuilder {
    pub fn new(
        num_grams: usize,
        _max_gram_id: usize,
        _max_count_rank: usize,
        _quantization_bits: usize,
    ) -> Self {
        Self {
            token_ids: Vec::with_capacity(num_grams),
            count_ranks: Vec::with_capacity(num_grams),
        }
    }

    pub fn add(&mut self, token_id: usize, count_rank: usize) {
        self.token_ids.push(token_id);
        self.count_ranks.push(count_rank);
    }

    pub fn add_count_rank(&mut self, rank: usize) {
        self.count_ranks.push(rank);
    }

    pub fn release(self, pointers: Vec<usize>) -> SimpleSortedArray {
        // let token_ids = SimpleGramsSequence::new(&self.token_ids, &pointers);
        let token_ids = SimpleGramsSequence::new(&self.token_ids);
        let count_ranks = self.count_ranks;
        SimpleSortedArray {
            token_ids,
            count_ranks,
            pointers,
        }
    }

    pub fn release_counts_ranks(self) -> SimpleSortedArray {
        SimpleSortedArray {
            token_ids: SimpleGramsSequence::default(),
            count_ranks: self.count_ranks,
            pointers: vec![],
        }
    }
}
