pub mod builder;

use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::grams_sequence::SimpleGramsSequence;
use crate::handle_bincode_error;

pub use crate::trie_array::builder::TrieArrayBuilder;

pub trait TrieArray {
    fn new(token_ids: Vec<usize>, count_ranks: Vec<usize>, pointers: Vec<usize>) -> Box<Self>;

    fn range(&self, pos: usize) -> (usize, usize);

    fn token_id(&self, pos: usize) -> usize;

    fn count_rank(&self, pos: usize) -> usize;

    fn position(&self, rng: (usize, usize), id: usize) -> Option<usize>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<()>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleTrieArray {
    token_ids: SimpleGramsSequence,
    count_ranks: Vec<usize>,
    pointers: Vec<usize>,
}

impl TrieArray for SimpleTrieArray {
    fn new(token_ids: Vec<usize>, count_ranks: Vec<usize>, pointers: Vec<usize>) -> Box<Self> {
        Box::new(Self {
            token_ids: SimpleGramsSequence::new(&token_ids),
            count_ranks,
            pointers,
        })
    }

    fn range(&self, pos: usize) -> (usize, usize) {
        (self.pointers[pos], self.pointers[pos + 1])
    }

    fn token_id(&self, pos: usize) -> usize {
        self.token_ids.get(pos)
    }

    fn count_rank(&self, pos: usize) -> usize {
        self.count_ranks[pos]
    }

    fn position(&self, rng: (usize, usize), id: usize) -> Option<usize> {
        self.token_ids.position(rng, id)
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
