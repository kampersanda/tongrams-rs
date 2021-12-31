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
    fn default() -> Box<Self> {
        Box::new(Self {
            map: HashMap::new(),
        })
    }

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        bincode::serialize_into(writer, self).map_err(handle_bincode_error)?;
        Ok(0)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let x: Self = bincode::deserialize_from(reader).map_err(handle_bincode_error)?;
        Ok(Box::new(x))
    }

    fn new(grams: &[Gram]) -> Result<Box<Self>> {
        let mut map = HashMap::new();
        for (id, gram) in grams.iter().enumerate() {
            if let Some(v) = map.insert(gram.to_string(), id) {
                return Err(anyhow!("Depulicated key: {:?} => {}", gram, v));
            }
        }
        Ok(Box::new(Self { map }))
    }

    fn get(&self, gram: Gram) -> Option<usize> {
        self.map.get(&gram.to_string()).copied()
    }
}
