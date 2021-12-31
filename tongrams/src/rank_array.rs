pub mod ef;
pub mod simple;

use std::io::{Read, Write};

use anyhow::Result;

pub use crate::rank_array::simple::SimpleRankArray;

pub trait RankArray {
    fn new(count_ranks: Vec<usize>) -> Box<Self>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<usize>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;

    fn size_in_bytes(&self) -> usize;

    fn memory_statistics(&self) -> serde_json::Value;

    fn get(&self, i: usize) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic<A: RankArray>() {
        let count_ranks = vec![3, 0, 0, 0, 1, 2, 0, 1, 1];
        let ra = A::new(count_ranks.clone());
        for (i, &x) in count_ranks.iter().enumerate() {
            assert_eq!(ra.get(i), x);
        }
    }

    #[test]
    fn test_basic_simple() {
        test_basic::<SimpleRankArray>();
    }
}
