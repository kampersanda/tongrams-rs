use std::collections::HashMap;
use std::io::{Read, Write};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::handle_bincode_error;
use crate::vocabulary::Vocabulary;
use crate::Gram;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SimpleVocabulary {
    map: HashMap<String, usize>,
}

impl Vocabulary for SimpleVocabulary {
    fn new() -> Box<Self> {
        Box::new(Self {
            map: HashMap::new(),
        })
    }

    fn build(grams: &[Gram]) -> Result<Box<Self>> {
        let mut map = HashMap::new();
        for (id, gram) in grams.iter().enumerate() {
            if let Some(v) = map.insert(gram.to_string(), id) {
                return Err(anyhow!("Depulicated key: {:?} => {}", gram, v));
            }
        }
        Ok(Box::new(Self { map }))
    }

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        bincode::serialize_into(writer, self).map_err(handle_bincode_error)?;
        Ok(self.size_in_bytes())
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let x: Self = bincode::deserialize_from(reader).map_err(handle_bincode_error)?;
        Ok(Box::new(x))
    }

    fn size_in_bytes(&self) -> usize {
        let mut bytes = vec![];
        bincode::serialize_into(&mut bytes, self)
            .map_err(handle_bincode_error)
            .unwrap();
        bytes.len()
    }

    fn memory_statistics(&self) -> serde_json::Value {
        serde_json::json!({})
    }

    fn get(&self, gram: Gram) -> Option<usize> {
        self.map.get(&gram.to_string()).copied()
    }
}
