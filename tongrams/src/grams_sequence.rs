#[derive(Default, Debug)]
pub struct SimpleGramsSequence {
    grams: Vec<usize>,
}

impl SimpleGramsSequence {
    pub fn new(grams: &[usize], _pointers: &[usize]) -> Self {
        Self {
            grams: grams.to_vec(),
        }
    }

    pub fn get(&self, i: usize) -> usize {
        self.grams[i]
    }

    pub fn find(&self, rng: (usize, usize), id: usize) -> Option<usize> {
        self.grams[rng.0..rng.1].iter().position(|&x| x == id)
    }
}
