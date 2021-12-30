use crate::vocabulary::SimpleVocabulary;
use crate::Gram;

use crate::MAX_ORDER;

#[derive(Default)]
pub struct SortedArrayMapper {
    mapped: [usize; MAX_ORDER],
}

impl SortedArrayMapper {
    pub fn map_query(&mut self, gram: Gram, vocab: &SimpleVocabulary) -> Option<&[usize]> {
        let tokens = gram.split_to_tokens();
        for (i, &w) in tokens.iter().enumerate() {
            if let Some(mapped_id) = vocab.get(w) {
                self.mapped[i] = mapped_id;
            } else {
                return None;
            }
        }
        Some(&self.mapped[..tokens.len()])
    }

    pub fn get(&self, i: usize) -> usize {
        self.mapped[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let grams = vec![
            Gram::from_str("A"),
            Gram::from_str("D"),
            Gram::from_str("B"),
        ];
        let vocab = SimpleVocabulary::new(&grams);
        let mut mapper = SortedArrayMapper::default();

        let gram = Gram::from_str("A B D");
        assert_eq!(mapper.map_query(gram, &vocab), Some(&[0, 2, 1][..]));

        let gram = Gram::from_str("E B");
        assert_eq!(mapper.map_query(gram, &vocab), None);
    }
}
