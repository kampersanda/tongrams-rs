pub mod simple;
pub mod yada;

use std::io::{Read, Write};

use anyhow::Result;

pub use crate::vocabulary::{simple::SimpleVocabulary, yada::DoubleArrayVocabulary};
use crate::Gram;

pub trait Vocabulary {
    fn default() -> Box<Self>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<usize>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;

    fn size_in_bytes(&self) -> usize;

    fn memory_statistics(&self) -> serde_json::Value;

    fn new(grams: &[Gram]) -> Result<Box<Self>>;

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

        let vocab = SimpleVocabulary::new(&grams).unwrap();
        assert_eq!(vocab.get(Gram::from_str("A")), Some(0));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(2));
        assert_eq!(vocab.get(Gram::from_str("C")), None);
        assert_eq!(vocab.get(Gram::from_str("D")), Some(1));

        let vocab = DoubleArrayVocabulary::new(&grams).unwrap();
        assert_eq!(vocab.get(Gram::from_str("A")), Some(0));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(2));
        assert_eq!(vocab.get(Gram::from_str("C")), None);
        assert_eq!(vocab.get(Gram::from_str("D")), Some(1));
    }
}
