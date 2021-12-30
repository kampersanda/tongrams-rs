use std::fs::File;
use std::io::Read;

use anyhow::Result;

use crate::loader::{GramsFileLoader, GramsLoader, GramsTextLoader};
use crate::mappers::SortedArrayMapper;
use crate::sequence_collection::{SequenceCollection, SequenceCollectionBuilder};
use crate::sorted_array::{SimpleSortedArray, SortedArrayBuilder};
use crate::vocabulary::SimpleVocabulary;
use crate::Gram;

#[derive(Default, Debug)]
pub struct TrieCountLm {
    max_order: usize,
    vocab: SimpleVocabulary,
    arrays: Vec<SimpleSortedArray>,
    counts: SequenceCollection,
}

impl TrieCountLm {
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
            Some(self.counts.access(order, count_rank))
        } else {
            None
        }
    }

    pub fn max_order(&self) -> usize {
        self.max_order
    }
}

pub struct TrieCountLmBuilder<R>
where
    R: Read,
{
    loaders: Vec<Box<dyn GramsLoader<R>>>,
    vocab: SimpleVocabulary,
    arrays: Vec<SimpleSortedArray>,
    counts_builder: SequenceCollectionBuilder,
}

impl<R> TrieCountLmBuilder<R>
where
    R: Read,
{
    pub fn new(loaders: Vec<Box<dyn GramsLoader<R>>>) -> Self {
        Self {
            loaders,
            vocab: SimpleVocabulary::default(),
            arrays: vec![],
            counts_builder: SequenceCollectionBuilder::default(),
        }
    }

    pub fn build(mut self) -> Result<TrieCountLm> {
        self.build_counts()?;
        self.build_vocabulary()?;

        let max_order = self.loaders.len() - 1;
        for order in 1..=max_order {
            self.build_sorted_array(order)?;
        }

        Ok(TrieCountLm {
            max_order,
            vocab: self.vocab,
            arrays: self.arrays,
            counts: SequenceCollection::new(self.counts_builder),
        })
    }

    fn build_counts(&mut self) -> Result<()> {
        eprintln!("build_counts");

        for loader in &self.loaders {
            let gp = loader.parser()?;
            for rec in gp {
                self.counts_builder.eat_value(rec?.count());
            }
            self.counts_builder.build_sequence();
        }

        Ok(())
    }

    fn build_vocabulary(&mut self) -> Result<()> {
        eprintln!("build_vocabulary");

        let records = {
            let gp = self.loaders[0].parser()?;
            let mut records = Vec::new();
            for rec in gp {
                let rec = rec?;
                records.push(rec);
            }
            records
        };

        let grams: Vec<Gram> = records.iter().map(|r| Gram::from_str(&r.gram)).collect();
        self.vocab = SimpleVocabulary::new(&grams);

        let mut sa_builder = SortedArrayBuilder::new(records.len(), 0, 0, 0);
        for rec in &records {
            let count_rank = self.counts_builder.rank(0, rec.count).unwrap();
            sa_builder.add_count_rank(count_rank);
        }

        let sa = sa_builder.release_counts_ranks();
        self.arrays.push(sa);

        Ok(())
    }

    /// Builds the sorted array of `order`.
    fn build_sorted_array(&mut self, order: usize) -> Result<()> {
        eprintln!("build_sorted_array (order={})", order);

        let mut prev_gp = self.loaders[order - 1].parser()?;
        let curr_gp = self.loaders[order].parser()?;

        let mut sa_builder = SortedArrayBuilder::new(curr_gp.num_grams(), 0, 0, 0);
        let num_pointers = prev_gp.num_grams() + 1;

        let mut pointers = Vec::with_capacity(num_pointers);
        pointers.push(0);

        let mut pointer = 0;
        let mut prev_rec = prev_gp.next().unwrap()?;

        for curr_rec in curr_gp {
            let curr_rec = curr_rec?;
            let (pattern, token) = curr_rec.gram().pop_token().unwrap(); // TODO: Error handling

            while pattern != prev_rec.gram() {
                pointers.push(pointer);
                if let Some(rec) = prev_gp.next() {
                    prev_rec = rec?;
                } else {
                    break;
                }
            }

            pointer += 1;

            let token_id = self.vocab.get(token).unwrap();
            let count_rank = self.counts_builder.rank(order, curr_rec.count()).unwrap();
            sa_builder.add(token_id, count_rank);
        }

        for _ in prev_gp {
            pointers.push(pointer);
        }
        pointers.push(pointer);

        let sa = sa_builder.release(pointers);
        self.arrays.push(sa);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let lm = TrieCountLm::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();

        #[allow(non_snake_case)]
        let (A, B, C, D) = (0, 1, 2, 3);

        // For vocab
        let vocab = &lm.vocab;
        assert_eq!(vocab.get(Gram::from_str("A")), Some(A));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(B));
        assert_eq!(vocab.get(Gram::from_str("C")), Some(C));
        assert_eq!(vocab.get(Gram::from_str("D")), Some(D));

        // For unigrams
        let sa = &lm.arrays[0];
        for (i, &count_rank) in [2, 1, 0, 0].iter().enumerate() {
            assert_eq!(sa.count_rank(i), count_rank);
        }

        // For bigrams
        let sa = &lm.arrays[1];
        for (i, &token_id) in [A, C, B, C, D, A, D, B, D].iter().enumerate() {
            assert_eq!(sa.token_id(i), token_id);
        }
        for (i, &count_rank) in [3, 0, 0, 0, 1, 2, 0, 1, 1].iter().enumerate() {
            assert_eq!(sa.count_rank(i), count_rank);
        }
        for (i, &range) in [(0, 2), (2, 5), (5, 7), (7, 9)].iter().enumerate() {
            assert_eq!(sa.range(i), range);
        }

        // For trigrams
        let sa = &lm.arrays[2];
        for (i, &token_id) in [C, C, D, D, B, C, D].iter().enumerate() {
            assert_eq!(sa.token_id(i), token_id);
        }
        for (i, &count_rank) in [2, 1, 0, 0, 1, 0, 0].iter().enumerate() {
            assert_eq!(sa.count_rank(i), count_rank);
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
            assert_eq!(sa.range(i), range);
        }
    }

    #[test]
    fn test_lookup() {
        let lm = TrieCountLm::from_texts(vec![GRAMS_1, GRAMS_2, GRAMS_3]).unwrap();

        eprintln!("{:?}", &lm);

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
    }
}
