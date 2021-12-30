use crate::grams_sequence::SimpleGramsSequence;
use crate::trie_array::SimpleTrieArray;

#[derive(Default)]
pub struct TrieLayerBuilder {
    token_ids: Vec<usize>,
    count_ranks: Vec<usize>,
}

impl TrieLayerBuilder {
    pub fn new(
        num_grams: usize,
        _max_gram_id: usize,
        _max_count_rank: usize,
        _quantization_bits: usize,
    ) -> Self {
        Self {
            token_ids: Vec::with_capacity(num_grams),
            count_ranks: Vec::with_capacity(num_grams),
        }
    }

    pub fn add(&mut self, token_id: usize, count_rank: usize) {
        self.token_ids.push(token_id);
        self.count_ranks.push(count_rank);
    }

    pub fn add_count_rank(&mut self, rank: usize) {
        self.count_ranks.push(rank);
    }

    pub fn release(self, pointers: Vec<usize>) -> SimpleTrieArray {
        // let token_ids = SimpleGramsSequence::new(&self.token_ids, &pointers);
        let token_ids = SimpleGramsSequence::new(&self.token_ids);
        let count_ranks = self.count_ranks;
        SimpleTrieArray {
            token_ids,
            count_ranks,
            pointers,
        }
    }

    pub fn release_counts_ranks(self) -> SimpleTrieArray {
        SimpleTrieArray {
            token_ids: SimpleGramsSequence::default(),
            count_ranks: self.count_ranks,
            pointers: vec![],
        }
    }
}
