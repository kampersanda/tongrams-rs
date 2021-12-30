pub mod builder;
pub mod ef;
pub mod simple;

use std::io::{Read, Write};

use anyhow::Result;

pub use crate::trie_array::builder::TrieArrayBuilder;
pub use crate::trie_array::ef::EliasFanoTrieArray;
pub use crate::trie_array::simple::SimpleTrieArray;

pub trait TrieArray {
    fn new(token_ids: Vec<usize>, count_ranks: Vec<usize>, pointers: Vec<usize>) -> Box<Self>;

    fn with_count_ranks(count_ranks: Vec<usize>) -> Box<Self>;

    fn token_id(&self, i: usize) -> usize;

    fn count_rank(&self, i: usize) -> usize;

    fn range(&self, pos: usize) -> (usize, usize);

    fn find_token(&self, pos: usize, id: usize) -> Option<usize>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<()>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_1<T: TrieArray>() {
        let token_ids = vec![0, 2, 1, 2, 3, 0, 3, 1, 3];
        let count_ranks = vec![3, 0, 0, 0, 1, 2, 0, 1, 1];
        let pointers = vec![0, 2, 5, 7, 9];
        let ta = T::new(token_ids.clone(), count_ranks.clone(), pointers.clone());

        for (i, &x) in token_ids.iter().enumerate() {
            assert_eq!(ta.token_id(i), x);
        }
        for (i, &x) in count_ranks.iter().enumerate() {
            assert_eq!(ta.count_rank(i), x);
        }
        for i in 0..pointers.len() - 1 {
            assert_eq!(ta.range(i), (pointers[i], pointers[i + 1]));
        }

        assert_eq!(ta.find_token(1, 3), Some(4));
        assert_eq!(ta.find_token(1, 1), Some(2));
        assert_eq!(ta.find_token(1, 4), None);
    }

    fn test_basic_2<T: TrieArray>() {
        let token_ids = vec![2, 2, 3, 3, 1, 2, 3];
        let count_ranks = vec![2, 1, 0, 0, 1, 0, 0];
        let pointers = vec![0, 1, 1, 3, 4, 4, 4, 4, 6, 7];
        let ta = T::new(token_ids.clone(), count_ranks.clone(), pointers.clone());

        for (i, &x) in token_ids.iter().enumerate() {
            assert_eq!(ta.token_id(i), x);
        }
        for (i, &x) in count_ranks.iter().enumerate() {
            assert_eq!(ta.count_rank(i), x);
        }
        for i in 0..pointers.len() - 1 {
            assert_eq!(ta.range(i), (pointers[i], pointers[i + 1]));
        }

        assert_eq!(ta.find_token(2, 2), Some(1));
        assert_eq!(ta.find_token(2, 3), Some(2));
        assert_eq!(ta.find_token(2, 4), None);
    }

    #[test]
    fn test_basic_simple() {
        test_basic_1::<SimpleTrieArray>();
        test_basic_2::<SimpleTrieArray>();
    }

    #[test]
    fn test_basic_ef() {
        test_basic_1::<EliasFanoTrieArray>();
        test_basic_2::<EliasFanoTrieArray>();
    }
}
