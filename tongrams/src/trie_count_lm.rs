pub mod builder;
pub mod lookuper;

use std::fs::File;
use std::io::{Read, Write};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::handle_bincode_error;
use crate::loader::{GramsFileLoader, GramsLoader, GramsTextLoader};
use crate::trie_array::TrieArray;
use crate::trie_count_lm::builder::TrieCountLmBuilder;
use crate::trie_count_lm::lookuper::TrieCountLmLookuper;
use crate::vocabulary::Vocabulary;

#[derive(Default, Debug)]
pub struct TrieCountLm<T, V>
where
    T: TrieArray,
    V: Vocabulary,
{
    vocab: V,
    arrays: Vec<T>,
    counts: Vec<Vec<usize>>,
}

impl<T, V> TrieCountLm<T, V>
where
    T: TrieArray,
    V: Vocabulary,
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

    pub fn lookuper(&self) -> TrieCountLmLookuper<T, V> {
        TrieCountLmLookuper::new(self)
    }

    pub fn num_orders(&self) -> usize {
        self.arrays.len()
    }

    pub fn serialize_into<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        self.vocab.serialize_into(&mut writer)?;
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
        let vocab = *V::deserialize_from(&mut reader)?;
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
            vocab,
            arrays,
            counts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EliasFanoTrieCountLm, Gram, SimpleTrieCountLm};

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

    const A: usize = 0;
    const B: usize = 1;
    const C: usize = 2;
    const D: usize = 3;

    fn test_vocabulary<V: Vocabulary>(vocab: &V) {
        assert_eq!(vocab.get(Gram::from_str("A")), Some(A));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(B));
        assert_eq!(vocab.get(Gram::from_str("C")), Some(C));
        assert_eq!(vocab.get(Gram::from_str("D")), Some(D));
    }

    fn test_unigrams<T: TrieArray>(array: &T) {
        for (i, &count_rank) in [2, 1, 0, 0].iter().enumerate() {
            assert_eq!(array.count_rank(i), count_rank);
        }
    }

    fn test_bigrams<T: TrieArray>(array: &T) {
        for (i, &token_id) in [A, C, B, C, D, A, D, B, D].iter().enumerate() {
            assert_eq!(array.token_id(i), token_id);
        }
        for (i, &count_rank) in [3, 0, 0, 0, 1, 2, 0, 1, 1].iter().enumerate() {
            assert_eq!(array.count_rank(i), count_rank);
        }
        for (i, &range) in [(0, 2), (2, 5), (5, 7), (7, 9)].iter().enumerate() {
            assert_eq!(array.range(i), range);
        }
    }

    fn test_trigrams<T: TrieArray>(array: &T) {
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
    fn test_simple_components() {
        let lm = SimpleTrieCountLm::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();
        test_vocabulary(&lm.vocab);
        test_unigrams(&lm.arrays[0]);
        test_bigrams(&lm.arrays[1]);
        test_trigrams(&lm.arrays[2]);
    }

    #[test]
    fn test_ef_components() {
        let lm = EliasFanoTrieCountLm::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();
        test_vocabulary(&lm.vocab);
        test_unigrams(&lm.arrays[0]);
        test_bigrams(&lm.arrays[1]);
        test_trigrams(&lm.arrays[2]);
    }

    #[test]
    fn test_simple_lookup() {
        let lm = SimpleTrieCountLm::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();
        let mut lookuper = lm.lookuper();

        let loader = GramsTextLoader::new(GRAMS_1.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }

        let loader = GramsTextLoader::new(GRAMS_2.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }

        let loader = GramsTextLoader::new(GRAMS_3.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }

        assert_eq!(lookuper.run(Gram::from_str("E")), None);
        assert_eq!(lookuper.run(Gram::from_str("B A")), None);
        assert_eq!(lookuper.run(Gram::from_str("B B A")), None);
    }

    #[test]
    fn test_ef_lookup() {
        let lm = EliasFanoTrieCountLm::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();
        let mut lookuper = lm.lookuper();

        let loader = GramsTextLoader::new(GRAMS_1.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }

        let loader = GramsTextLoader::new(GRAMS_2.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }

        let loader = GramsTextLoader::new(GRAMS_3.as_bytes());
        let gp = loader.parser().unwrap();
        for rec in gp {
            let rec = rec.unwrap();
            assert_eq!(lookuper.run(rec.gram()), Some(rec.count()));
        }

        assert_eq!(lookuper.run(Gram::from_str("E")), None);
        assert_eq!(lookuper.run(Gram::from_str("B A")), None);
        assert_eq!(lookuper.run(Gram::from_str("B B A")), None);
    }
}
