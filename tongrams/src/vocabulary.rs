pub mod simple;
pub mod yada;

use std::io::{Read, Write};

use anyhow::Result;

pub use crate::vocabulary::{simple::SimpleVocabulary, yada::DoubleArrayVocabulary};
use crate::Gram;

pub trait Vocabulary {
    fn new() -> Box<Self>;

    fn build(grams: &[Gram]) -> Result<Box<Self>>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<usize>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;

    fn size_in_bytes(&self) -> usize;

    fn memory_statistics(&self) -> serde_json::Value;

    fn get(&self, gram: Gram) -> Option<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let grams = vec![
            Gram::from_str("A"),
            Gram::from_str("D"),
            Gram::from_str("B"),
        ];

        let vocab = SimpleVocabulary::build(&grams).unwrap();
        assert_eq!(vocab.get(Gram::from_str("A")), Some(0));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(2));
        assert_eq!(vocab.get(Gram::from_str("C")), None);
        assert_eq!(vocab.get(Gram::from_str("D")), Some(1));

        let vocab = DoubleArrayVocabulary::build(&grams).unwrap();
        assert_eq!(vocab.get(Gram::from_str("A")), Some(0));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(2));
        assert_eq!(vocab.get(Gram::from_str("C")), None);
        assert_eq!(vocab.get(Gram::from_str("D")), Some(1));
    }
}
