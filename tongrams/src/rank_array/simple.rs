use std::io::{Read, Write};

use anyhow::Result;

use crate::rank_array::RankArray;

/// Simple implementation of [`RankArray`] with `Vec<usize>`.
#[derive(Default, Debug)]
pub struct SimpleRankArray {
    count_ranks: Vec<usize>,
}

impl RankArray for SimpleRankArray {
    fn build(count_ranks: Vec<usize>) -> Box<Self> {
        Box::new(Self { count_ranks })
    }

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        sucds::util::vec_io::serialize_usize(&self.count_ranks, writer)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let count_ranks = sucds::util::vec_io::deserialize_usize(reader)?;
        Ok(Box::new(Self { count_ranks }))
    }

    fn size_in_bytes(&self) -> usize {
        sucds::util::vec_io::size_in_bytes(&self.count_ranks)
    }

    fn memory_statistics(&self) -> serde_json::Value {
        let count_ranks = sucds::util::vec_io::size_in_bytes(&self.count_ranks);
        serde_json::json!({ "count_ranks": count_ranks })
    }

    fn get(&self, i: usize) -> usize {
        self.count_ranks[i]
    }

    fn len(&self) -> usize {
        self.count_ranks.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
