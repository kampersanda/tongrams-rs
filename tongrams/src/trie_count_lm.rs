pub mod builder;

use std::fs::File;
use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::handle_bincode_error;
use crate::loader::{GramsFileLoader, GramsLoader, GramsTextLoader};
use crate::mappers::SortedArrayMapper;
use crate::trie_array::TrieArray;
use crate::trie_count_lm::builder::TrieCountLmBuilder;
use crate::vocabulary::SimpleVocabulary;
use crate::Gram;

#[derive(Default, Debug)]
pub struct TrieCountLm<T>
where
    T: TrieArray,
{
    max_order: usize,
    vocab: SimpleVocabulary,
    arrays: Vec<T>,
    counts: Vec<Vec<usize>>,
}

impl<T> TrieCountLm<T>
where
    T: TrieArray,
{
    pub fn from_files(filenames: Vec<String>) -> Result<Self> {
        let mut loaders = Vec::with_capacity(filenames.len());
        for filename in filenames {
            let loader: Box<dyn GramsLoader<File>> = Box::new(GramsFileLoader::new(filename));
            loaders.push(loader);
        }
        TrieCountLmBuilder::new(loaders).build()
    }

    pub fn from_texts(texts: Vec<&'static str>) -> Result<Self> {
        let mut loaders = Vec::with_capacity(texts.len());
        for text in texts {
            let loader: Box<dyn GramsLoader<&[u8]>> =
                Box::new(GramsTextLoader::new(text.as_bytes()));
            loaders.push(loader);
        }
        TrieCountLmBuilder::new(loaders).build()
    }

    pub fn lookup(&self, gram: Gram) -> Option<usize> {
        let mut mapper = SortedArrayMapper::default();
        if let Some(token_ids) = mapper.map_query(gram, &self.vocab) {
            let order = token_ids.len() - 1;
            let mut pos = token_ids[0];
            for i in 1..=order {
                let rng = self.arrays[i].range(pos);
                if let Some(next_pos) = self.arrays[i].position(rng, token_ids[i]) {
                    pos = next_pos;
                } else {
                    return None;
                }
            }
            let count_rank = self.arrays[order].count_rank(pos);
            Some(self.counts[order][count_rank])
        } else {
            None
        }
    }

    pub fn max_order(&self) -> usize {
        self.max_order
    }

    pub fn serialize_into<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        bincode::serialize_into(&mut writer, &self.max_order).map_err(handle_bincode_error)?;
        bincode::serialize_into(&mut writer, &self.vocab).map_err(handle_bincode_error)?;
        writer.write_u64::<LittleEndian>(self.arrays.len() as u64)?;
        for array in &self.arrays {
            array.serialize_into(&mut writer)?;
        }
        bincode::serialize_into(&mut writer, &self.counts).map_err(handle_bincode_error)?;
        Ok(())
    }

    pub fn deserialize_from<R>(mut reader: R) -> Result<Self>
    where
        R: Read,
    {
        let max_order = bincode::deserialize_from(&mut reader).map_err(handle_bincode_error)?;
        let vocab = bincode::deserialize_from(&mut reader).map_err(handle_bincode_error)?;
        let arrays = {
            let len = reader.read_u64::<LittleEndian>()? as usize;
            let mut arrays = Vec::with_capacity(len);
            for _ in 0..len {
                arrays.push(*T::deserialize_from(&mut reader)?);
            }
            arrays
        };
        let counts = bincode::deserialize_from(&mut reader).map_err(handle_bincode_error)?;
        Ok(Self {
            max_order,
            vocab,
            arrays,
            counts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimpleTrieArray;

    const GRAMS_1: &'static str = "4
A\t10
B\t7
C\t1
D\t1
";

    const GRAMS_2: &'static str = "9
A A\t5
A C\t2
B B\t2
B C\t2
B D\t1
C A\t3
C D\t2
D B\t1
D D\t1
";

    const GRAMS_3: &'static str = "7
A A C\t4
B B C\t2
B B D\t1
B C D\t1
D B B\t2
D B C\t1
D D D\t1
";

    #[test]
    fn test_components() {
        let lm =
            TrieCountLm::<SimpleTrieArray>::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();

        #[allow(non_snake_case)]
        let (A, B, C, D) = (0, 1, 2, 3);

        // For vocab
        let vocab = &lm.vocab;
        assert_eq!(vocab.get(Gram::from_str("A")), Some(A));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(B));
        assert_eq!(vocab.get(Gram::from_str("C")), Some(C));
        assert_eq!(vocab.get(Gram::from_str("D")), Some(D));

        // For unigrams
        let array = &lm.arrays[0];
        for (i, &count_rank) in [2, 1, 0, 0].iter().enumerate() {
            assert_eq!(array.count_rank(i), count_rank);
        }

        // For bigrams
        let array = &lm.arrays[1];
        for (i, &token_id) in [A, C, B, C, D, A, D, B, D].iter().enumerate() {
            assert_eq!(array.token_id(i), token_id);
        }
        for (i, &count_rank) in [3, 0, 0, 0, 1, 2, 0, 1, 1].iter().enumerate() {
            assert_eq!(array.count_rank(i), count_rank);
        }
        for (i, &range) in [(0, 2), (2, 5), (5, 7), (7, 9)].iter().enumerate() {
            assert_eq!(array.range(i), range);
        }

        // For trigrams
        let array = &lm.arrays[2];
        for (i, &token_id) in [C, C, D, D, B, C, D].iter().enumerate() {
            assert_eq!(array.token_id(i), token_id);
        }
        for (i, &count_rank) in [2, 1, 0, 0, 1, 0, 0].iter().enumerate() {
            assert_eq!(array.count_rank(i), count_rank);
        }
        for (i, &range) in [
            (0, 1),
            (1, 1),
            (1, 3),
            (3, 4),
            (4, 4),
            (4, 4),
            (4, 4),
            (4, 6),
            (6, 7),
        ]
        .iter()
        .enumerate()
        {
            assert_eq!(array.range(i), range);
        }
    }

    #[test]
    fn test_lookup() {
        let lm =
            TrieCountLm::<SimpleTrieArray>::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();

        let loader = GramsTextLoader::new(GRAMS_1.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lm.lookup(rec.gram()), Some(rec.count()));
        }

        let loader = GramsTextLoader::new(GRAMS_2.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lm.lookup(rec.gram()), Some(rec.count()));
        }

        let loader = GramsTextLoader::new(GRAMS_3.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lm.lookup(rec.gram()), Some(rec.count()));
        }

        assert_eq!(lm.lookup(Gram::from_str("E")), None);
        assert_eq!(lm.lookup(Gram::from_str("B A")), None);
        assert_eq!(lm.lookup(Gram::from_str("B B A")), None);
    }
}
