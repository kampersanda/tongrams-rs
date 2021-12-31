pub mod ef;
pub mod simple;

use std::io::{Read, Write};

use anyhow::Result;

pub use crate::trie_array::ef::EliasFanoTrieArray;
pub use crate::trie_array::simple::SimpleTrieArray;

pub trait TrieArray {
    fn build(token_ids: Vec<usize>, pointers: Vec<usize>) -> Box<Self>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<usize>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;

    fn size_in_bytes(&self) -> usize;

    fn memory_statistics(&self) -> serde_json::Value;

    fn token_id(&self, i: usize) -> usize;

    fn range(&self, pos: usize) -> (usize, usize);

    fn find_token(&self, pos: usize, id: usize) -> Option<usize>;

    fn num_tokens(&self) -> usize;

    fn num_pointers(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_basic_1<T: TrieArray>() {
        let token_ids = vec![0, 2, 1, 2, 3, 0, 3, 1, 3];
        let pointers = vec![0, 2, 5, 7, 9];
        let ta = T::build(token_ids.clone(), pointers.clone());

        for (i, &x) in token_ids.iter().enumerate() {
            assert_eq!(ta.token_id(i), x);
        }
        for i in 0..pointers.len() - 1 {
            assert_eq!(ta.range(i), (pointers[i], pointers[i + 1]));
        }

        assert_eq!(ta.find_token(1, 3), Some(4));
        assert_eq!(ta.find_token(1, 1), Some(2));
        assert_eq!(ta.find_token(1, 4), None);

        assert_eq!(ta.num_tokens(), 9);
        assert_eq!(ta.num_pointers(), 5);
    }

    fn test_basic_2<T: TrieArray>() {
        let token_ids = vec![2, 2, 3, 3, 1, 2, 3];
        let pointers = vec![0, 1, 1, 3, 4, 4, 4, 4, 6, 7];
        let ta = T::build(token_ids.clone(), pointers.clone());

        for (i, &x) in token_ids.iter().enumerate() {
            assert_eq!(ta.token_id(i), x);
        }
        for i in 0..pointers.len() - 1 {
            assert_eq!(ta.range(i), (pointers[i], pointers[i + 1]));
        }

        assert_eq!(ta.find_token(2, 2), Some(1));
        assert_eq!(ta.find_token(2, 3), Some(2));
        assert_eq!(ta.find_token(2, 4), None);

        assert_eq!(ta.num_tokens(), 7);
        assert_eq!(ta.num_pointers(), 10);
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
