use std::io::{Read, Write};

use anyhow::Result;

use crate::rank_array::RankArray;

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
        sucds::util::int_vector::serialize_into(&self.count_ranks, writer)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let count_ranks = sucds::util::int_vector::deserialize_from(reader)?;
        Ok(Box::new(Self { count_ranks }))
    }

    fn size_in_bytes(&self) -> usize {
        sucds::util::int_vector::size_in_bytes(&self.count_ranks)
    }

    fn memory_statistics(&self) -> serde_json::Value {
        let count_ranks = sucds::util::int_vector::size_in_bytes(&self.count_ranks);
        serde_json::json!({ "count_ranks": count_ranks })
    }

    fn get(&self, i: usize) -> usize {
        self.count_ranks[i]
    }
}
