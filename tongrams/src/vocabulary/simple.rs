use std::collections::HashMap;
use std::io::{Read, Write};

use anyhow::{anyhow, Result};

use crate::handle_bincode_error;
use crate::vocabulary::Vocabulary;
use crate::Gram;

/// Simple implementation of [`Vocabulary`] with `HashMap`.
#[derive(Default, Debug)]
pub struct SimpleVocabulary {
    map: HashMap<String, usize>,
}

impl Vocabulary for SimpleVocabulary {
    fn new() -> Box<Self> {
        Box::new(Self {
            map: HashMap::new(),
        })
    }

    fn build(tokens: &[Gram]) -> Result<Box<Self>> {
        let mut map = HashMap::new();
        for (id, token) in tokens.iter().enumerate() {
            if let Some(v) = map.insert(token.to_string(), id) {
                return Err(anyhow!("Depulicated key: {:?} => {}", token, v));
            }
        }
        Ok(Box::new(Self { map }))
    }

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        bincode::serialize_into(writer, &self.map).map_err(handle_bincode_error)?;
        Ok(self.size_in_bytes())
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let map = bincode::deserialize_from(reader).map_err(handle_bincode_error)?;
        Ok(Box::new(Self { map }))
    }

    fn size_in_bytes(&self) -> usize {
        bincode::serialize(&self.map)
            .map_err(handle_bincode_error)
            .unwrap()
            .len()
    }

    fn memory_statistics(&self) -> serde_json::Value {
        serde_json::json!({})
    }

    fn get(&self, token: Gram) -> Option<usize> {
        self.map.get(&token.to_string()).copied()
    }
}
