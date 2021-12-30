use std::collections::HashMap;
use std::io::Read;

use anyhow::Result;

use crate::loader::GramsLoader;
use crate::sorted_array::{SimpleSortedArray, SortedArrayBuilder};
use crate::vocabulary::SimpleVocabulary;
use crate::Gram;
use crate::TrieCountLm;

pub struct TrieCountLmBuilder<R>
where
    R: Read,
{
    loaders: Vec<Box<dyn GramsLoader<R>>>,
    vocab: SimpleVocabulary,
    arrays: Vec<SimpleSortedArray>,
    counts_builder: CountsBuilder,
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
            counts_builder: CountsBuilder::default(),
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
            counts: self.counts_builder.release(),
        })
    }

    fn build_counts(&mut self) -> Result<()> {
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

pub struct CountsBuilder {
    // Mapping from eaten values to their frequencies
    v2f_map: HashMap<usize, usize>,
    // Mappings from eaten values to their ranks
    v2r_maps: Vec<HashMap<usize, usize>>,
    // In which values are sorted in decreasing order of their frequencies
    sorted_sequences: Vec<Vec<usize>>,
}

impl Default for CountsBuilder {
    fn default() -> Self {
        Self {
            v2f_map: HashMap::new(),
            v2r_maps: vec![],
            sorted_sequences: vec![],
        }
    }
}

impl CountsBuilder {
    pub fn release(self) -> Vec<Vec<usize>> {
        self.sorted_sequences
    }

    pub fn eat_value(&mut self, x: usize) {
        if let Some(e) = self.v2f_map.get_mut(&x) {
            *e += 1;
        } else {
            self.v2f_map.insert(x, 1);
        }
    }

    /// Builds the sequence of the current order.
    pub fn build_sequence(&mut self) {
        if self.v2f_map.is_empty() {
            self.v2r_maps.push(HashMap::new());
            self.sorted_sequences.push(vec![]);
            return;
        }

        let mut sorted = vec![];
        for (&value, &freq) in &self.v2f_map {
            sorted.push((value, freq));
        }
        self.v2f_map.clear();

        // `then_with` is needed to stably sort
        sorted.sort_by(|(v1, f1), (v2, f2)| f2.cmp(f1).then_with(|| v1.cmp(v2)));
        self.sorted_sequences
            .push(sorted.iter().map(|&(v, _)| v).collect());

        let mut v2r_map = HashMap::new();
        for (i, &(v, _)) in sorted.iter().enumerate() {
            v2r_map.insert(v, i);
        }
        self.v2r_maps.push(v2r_map);
    }

    pub fn rank(&self, order: usize, value: usize) -> Option<usize> {
        self.v2r_maps[order].get(&value).map(|x| *x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let seqs = vec![vec![2, 2, 1, 2, 4, 2, 1, 2, 1], vec![2, 1, 2, 1, 1, 1]];

        let mut scb = CountsBuilder::default();
        for seq in &seqs {
            for &x in seq {
                scb.eat_value(x);
            }
            scb.build_sequence();
        }

        assert_eq!(scb.rank(0, 1), Some(1));
        assert_eq!(scb.rank(0, 2), Some(0));
        assert_eq!(scb.rank(0, 3), None);
        assert_eq!(scb.rank(0, 4), Some(2));
        assert_eq!(scb.rank(1, 1), Some(0));
        assert_eq!(scb.rank(1, 2), Some(1));

        let counts = scb.release();
        assert_eq!(counts[0][0], 2);
        assert_eq!(counts[0][1], 1);
        assert_eq!(counts[0][2], 4);
        assert_eq!(counts[1][0], 1);
        assert_eq!(counts[1][1], 2);
    }
}
