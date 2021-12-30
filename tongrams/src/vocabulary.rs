use std::collections::HashMap;
use std::io::{Read, Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::handle_bincode_error;
use crate::Gram;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleVocabulary {
    map: HashMap<String, usize>,
}

impl SimpleVocabulary {
    pub fn new(grams: &[Gram]) -> Self {
        let mut map = HashMap::new();
        for (id, gram) in grams.iter().enumerate() {
            map.insert(gram.to_string(), id);
        }
        Self { map }
    }

    pub fn get(&self, gram: Gram) -> Option<usize> {
        self.map.get(&gram.to_string()).map(|x| *x)
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
