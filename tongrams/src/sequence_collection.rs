use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct SequenceCollection {
    sorted_sequences: Vec<Vec<usize>>,
}

impl SequenceCollection {
    pub fn new(b: SequenceCollectionBuilder) -> Self {
        Self {
            sorted_sequences: b.sorted_sequences,
        }
    }

    /// Gets the `i`-th value in the frequency order.
    pub fn access(&self, order: usize, i: usize) -> usize {
        self.sorted_sequences[order][i]
    }
}

pub struct SequenceCollectionBuilder {
    // Mapping from eaten values to their frequencies
    v2f_map: HashMap<usize, usize>,
    // Mappings from eaten values to their ranks
    v2r_maps: Vec<HashMap<usize, usize>>,
    // In which values are sorted in decreasing order of their frequencies
    sorted_sequences: Vec<Vec<usize>>,
}

impl Default for SequenceCollectionBuilder {
    fn default() -> Self {
        Self {
            v2f_map: HashMap::new(),
            v2r_maps: vec![],
            sorted_sequences: vec![],
        }
    }
}

impl SequenceCollectionBuilder {
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

        let mut scb = SequenceCollectionBuilder::default();
        for seq in &seqs {
            for &x in seq {
                scb.eat_value(x);
            }
            scb.build_sequence();
        }

        eprintln!("v2r_maps = {:?}", scb.v2r_maps);
        eprintln!("sorted_sequences = {:?}", scb.sorted_sequences);

        assert_eq!(scb.rank(0, 1), Some(1));
        assert_eq!(scb.rank(0, 2), Some(0));
        assert_eq!(scb.rank(0, 3), None);
        assert_eq!(scb.rank(0, 4), Some(2));
        assert_eq!(scb.rank(1, 1), Some(0));
        assert_eq!(scb.rank(1, 2), Some(1));

        let sc = SequenceCollection::new(scb);
        assert_eq!(sc.access(0, 0), 2);
        assert_eq!(sc.access(0, 1), 1);
        assert_eq!(sc.access(0, 2), 4);
        assert_eq!(sc.access(1, 0), 1);
        assert_eq!(sc.access(1, 1), 2);
    }
}
