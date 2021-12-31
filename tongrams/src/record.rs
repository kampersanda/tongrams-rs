use crate::Gram;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    gram: String, // TODO: Store as a byte slice to another buffer
    count: usize,
}

impl Record {
    pub const fn new(gram: String, count: usize) -> Self {
        Self { gram, count }
    }

    pub fn gram(&self) -> Gram {
        Gram::new(self.gram.as_bytes())
    }

    pub const fn count(&self) -> usize {
        self.count
    }
}
