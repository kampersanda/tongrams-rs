use std::io::{Read, Write};

use anyhow::{anyhow, Result};
use sucds::util::int_vector;
use yada::{builder::DoubleArrayBuilder, DoubleArray};

use crate::vocabulary::Vocabulary;
use crate::Gram;

#[derive(Default, Debug)]
pub struct DoubleArrayVocabulary {
    data: Vec<u8>,
}

impl Vocabulary for DoubleArrayVocabulary {
    fn default() -> Box<Self> {
        Box::new(Self { data: Vec::new() })
    }

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        int_vector::serialize_into(&self.data, writer)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let data = int_vector::deserialize_from(reader)?;
        Ok(Box::new(Self { data }))
    }

    fn size_in_bytes(&self) -> usize {
        int_vector::size_in_bytes(&self.data)
    }

    fn memory_statistics(&self) -> serde_json::Value {
        let data = int_vector::size_in_bytes(&self.data);
        serde_json::json!({ "data": data })
    }

    fn new(grams: &[Gram]) -> Result<Box<Self>> {
        if (grams.len() >> 31) != 0 {
            return Err(anyhow!(
                "The number of grams must be represented in 31 bits."
            ));
        }

        let mut keyset = vec![];
        for (id, gram) in grams.iter().enumerate() {
            keyset.push((gram.raw(), id as u32));
        }
        keyset.sort_by(|(g1, _), (g2, _)| g1.cmp(g2));

        for i in 1..keyset.len() {
            if keyset[i - 1].0 == keyset[i].0 {
                let (k, v) = keyset[i - 1];
                return Err(anyhow!("Depulicated key: {:?} => {}", k, v));
            }
        }

        Ok(Box::new(Self {
            data: DoubleArrayBuilder::build(&keyset[..]).unwrap(),
        }))
    }

    fn get(&self, gram: Gram) -> Option<usize> {
        let da = DoubleArray::new(&self.data[..]);
        da.exact_match_search(gram.raw()).map(|x| x as usize)
    }
}
