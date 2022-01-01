use crate::vocabulary::Vocabulary;
use crate::Gram;

use crate::MAX_ORDER;

#[derive(Default)]
pub struct SortedArrayMapper {
    mapped: [usize; MAX_ORDER],
}

impl SortedArrayMapper {
    #[inline(always)]
    pub fn map_query<V>(&mut self, gram: Gram, vocab: &V) -> Option<&[usize]>
    where
        V: Vocabulary,
    {
        let tokens = gram.split_to_tokens();
        if MAX_ORDER < tokens.len() {
            return None;
        }
        for (i, &w) in tokens.iter().enumerate() {
            if let Some(mapped_id) = vocab.get(w) {
                self.mapped[i] = mapped_id;
            } else {
                return None;
            }
        }
        Some(&self.mapped[..tokens.len()])
    }

    #[allow(dead_code)]
    pub const fn get(&self, i: usize) -> usize {
        self.mapped[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocabulary::SimpleVocabulary;

    #[test]
    fn test_basic() {
        let grams = vec![
            Gram::from_str("A"),
            Gram::from_str("D"),
            Gram::from_str("B"),
        ];
        let vocab = *SimpleVocabulary::build(&grams).unwrap();
        let mut mapper = SortedArrayMapper::default();

        let gram = Gram::from_str("A B D");
        assert_eq!(mapper.map_query(gram, &vocab), Some(&[0, 2, 1][..]));

        let gram = Gram::from_str("E B");
        assert_eq!(mapper.map_query(gram, &vocab), None);
    }
}
