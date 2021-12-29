use std::collections::HashMap;

use crate::Gram;

#[derive(Default, Debug)]
pub struct SimpleVocabulary {
    map: HashMap<String, usize>,
}

impl SimpleVocabulary {
    pub fn new(grams: &[Gram]) -> Self {
        let mut map = HashMap::new();
        for (id, gram) in grams.iter().enumerate() {
            map.insert(gram.to_string(), id);
        }
        Self { map }
    }

    pub fn get(&self, gram: Gram) -> Option<usize> {
        self.map.get(&gram.to_string()).map(|x| *x)
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
        assert_eq!(vocab.get(Gram::from_str("A")), Some(0));
        assert_eq!(vocab.get(Gram::from_str("B")), Some(2));
        assert_eq!(vocab.get(Gram::from_str("C")), None);
        assert_eq!(vocab.get(Gram::from_str("D")), Some(1));
    }
}
