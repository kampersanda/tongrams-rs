use crate::trie_array::TrieArray;

#[derive(Default)]
pub struct TrieArrayBuilder {
    token_ids: Vec<usize>,
    count_ranks: Vec<usize>,
}

impl TrieArrayBuilder {
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

    pub fn release<T: TrieArray>(self, pointers: Vec<usize>) -> T {
        *T::new(self.token_ids, self.count_ranks, pointers)
    }

    pub fn release_counts_ranks<T: TrieArray>(self) -> T {
        *T::with_count_ranks(self.count_ranks)
    }
}
