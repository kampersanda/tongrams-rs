use crate::mappers::SortedArrayMapper;
use crate::trie_array::TrieArray;
use crate::trie_count_lm::TrieCountLm;
use crate::vocabulary::Vocabulary;
use crate::Gram;

pub struct TrieCountLmLookuper<'a, T, V>
where
    T: TrieArray,
    V: Vocabulary,
{
    trie: &'a TrieCountLm<T, V>,
    mapper: SortedArrayMapper,
}

impl<'a, T, V> TrieCountLmLookuper<'a, T, V>
where
    T: TrieArray,
    V: Vocabulary,
{
    pub fn new(trie: &'a TrieCountLm<T, V>) -> TrieCountLmLookuper<'a, T, V> {
        TrieCountLmLookuper {
            trie,
            mapper: SortedArrayMapper::default(),
        }
    }

    pub fn run(&mut self, gram: Gram) -> Option<usize> {
        if let Some(token_ids) = self.mapper.map_query(gram, &self.trie.vocab) {
            let order = token_ids.len() - 1;
            let mut pos = token_ids[0];
            for (&token_id, array) in token_ids[1..].iter().zip(self.trie.arrays[1..].iter()) {
                let rng = array.range(pos);
                if let Some(next_pos) = array.position(rng, token_id) {
                    pos = next_pos;
                } else {
                    return None;
                }
            }
            let count_rank = self.trie.arrays[order].count_rank(pos);
            Some(self.trie.counts[order][count_rank])
        } else {
            None
        }
    }
}
