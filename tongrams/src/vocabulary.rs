use std::collections::HashMap;
use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::handle_bincode_error;
use crate::Gram;

pub trait Vocabulary {
    fn default() -> Box<Self>;

    fn new(grams: &[Gram]) -> Box<Self>;

    fn get(&self, gram: Gram) -> Option<usize>;

    fn serialize_into<W: Write>(&self, writer: W) -> Result<()>;

    fn deserialize_from<R: Read>(reader: R) -> Result<Box<Self>>;
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleVocabulary {
    map: HashMap<String, usize>,
}

impl Vocabulary for SimpleVocabulary {
    fn default() -> Box<Self> {
        Box::new(Self {
            map: HashMap::new(),
        })
    }

    fn new(grams: &[Gram]) -> Box<Self> {
        let mut map = HashMap::new();
        for (id, gram) in grams.iter().enumerate() {
            map.insert(gram.to_string(), id);
        }
        Box::new(Self { map })
    }

    fn get(&self, gram: Gram) -> Option<usize> {
        self.map.get(&gram.to_string()).map(|x| *x)
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
        let vocab = SimpleVocabulary::new(&grams);
        assert_eq!(vocab.get(Gram::from_str("A")), Some(0));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(2));
        assert_eq!(vocab.get(Gram::from_str("C")), None);
        assert_eq!(vocab.get(Gram::from_str("D")), Some(1));
    }
}
