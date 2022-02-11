use crate::Gram;

/// Handler of a pair of a gram and its count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CountRecord {
    gram: String, // TODO: Store as a byte slice to another buffer
    count: usize,
}

impl CountRecord {
    /// Creates a [`CountRecord`].
    #[inline]
    pub const fn new(gram: String, count: usize) -> Self {
        Self { gram, count }
    }

    /// Gets the [`Gram`].
    #[inline]
    pub fn gram(&self) -> Gram {
        Gram::new(self.gram.as_bytes())
    }

    /// Gets the count.
    #[inline]
    pub const fn count(&self) -> usize {
        self.count
    }
}
