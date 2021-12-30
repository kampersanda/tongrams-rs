#[derive(Default, Debug)]
pub struct SimpleGramsSequence {
    token_ids: Vec<usize>,
}

impl SimpleGramsSequence {
    pub fn new(token_ids: &[usize]) -> Self {
        Self {
            token_ids: token_ids.to_vec(),
        }
    }

    /// Gets the token id with a given index.
    ///
    /// ```
    /// use tongrams::SimpleGramsSequence;
    ///
    /// let token_ids = vec![4, 1, 2, 4, 3, 2, 2, 4];
    /// let seq = SimpleGramsSequence::new(&token_ids);
    /// assert_eq!(seq.get(1), 1);
    /// assert_eq!(seq.get(3), 4);
    /// ```
    pub fn get(&self, i: usize) -> usize {
        self.token_ids[i]
    }

    /// Searches for an element within a given range, returning its index.
    ///
    /// ```
    /// use tongrams::SimpleGramsSequence;
    ///
    /// let token_ids = vec![4, 1, 2, 4, 3, 2, 2, 4];
    /// let seq = SimpleGramsSequence::new(&token_ids);
    /// assert_eq!(seq.position((1, 4), 2), Some(2));
    /// assert_eq!(seq.position((1, 4), 3), None);
    /// ```
    pub fn position(&self, rng: (usize, usize), id: usize) -> Option<usize> {
        self.token_ids[rng.0..rng.1]
            .iter()
            .position(|&x| x == id)
            .map(|i| i + rng.0)
    }
}
