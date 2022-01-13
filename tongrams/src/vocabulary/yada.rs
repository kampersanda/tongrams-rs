use std::io::{Read, Write};

use anyhow::{anyhow, Result};
use yada::{builder::DoubleArrayBuilder, DoubleArray};

use crate::vocabulary::Vocabulary;
use crate::Gram;

/// Compact double-array implementation of [`Vocabulary`].
#[derive(Default, Debug)]
pub struct DoubleArrayVocabulary {
    data: Vec<u8>,
}

impl Vocabulary for DoubleArrayVocabulary {
    fn new() -> Box<Self> {
        Box::new(Self { data: Vec::new() })
    }

    fn build(tokens: &[Gram]) -> Result<Box<Self>> {
        if (tokens.len() >> 31) != 0 {
            return Err(anyhow!(
                "The number of tokens must be represented in 31 bits."
            ));
        }

        let mut keyset = vec![];
        for (id, token) in tokens.iter().enumerate() {
            keyset.push((token.raw(), id as u32));
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

    fn serialize_into<W>(&self, writer: W) -> Result<usize>
    where
        W: Write,
    {
        sucds::util::vec_io::serialize_u8(&self.data, writer)
    }

    fn deserialize_from<R>(reader: R) -> Result<Box<Self>>
    where
        R: Read,
    {
        let data = sucds::util::vec_io::deserialize_u8(reader)?;
        Ok(Box::new(Self { data }))
    }

    fn size_in_bytes(&self) -> usize {
        sucds::util::vec_io::size_in_bytes(&self.data)
    }

    fn memory_statistics(&self) -> serde_json::Value {
        let data = sucds::util::vec_io::size_in_bytes(&self.data);
        serde_json::json!({ "data": data })
    }

    #[inline(always)]
    fn get(&self, token: Gram) -> Option<usize> {
        let da = DoubleArray::new(&self.data[..]);
        da.exact_match_search(token.raw()).map(|x| x as usize)
    }
}
