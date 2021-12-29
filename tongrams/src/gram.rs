use std::fmt;

use crate::GRAM_SEPARATOR;

#[derive(Clone, Copy, Eq)]
pub struct Gram<'a> {
    data: &'a [u8],
}

impl<'a> Gram<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn from_str(data: &'a str) -> Self {
        Self {
            data: data.as_bytes(),
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }

    pub fn raw(&self) -> &[u8] {
        self.data
    }

    /// Pops the last word.
    ///
    /// ```
    /// use tongrams::Gram;
    ///
    /// let words = "abc de f";
    /// let mut gram = Gram::from_str(words);
    ///
    /// let (gram, last) = gram.pop_token().unwrap();
    /// assert_eq!(gram.raw(), "abc de".as_bytes());
    /// assert_eq!(last.raw(), "f".as_bytes());
    ///
    /// let (gram, last) = gram.pop_token().unwrap();
    /// assert_eq!(gram.raw(), "abc".as_bytes());
    /// assert_eq!(last.raw(), "de".as_bytes());
    ///
    /// assert_eq!(gram.pop_token(), None);
    /// ```
    pub fn pop_token(&self) -> Option<(Self, Self)> {
        let data = self.data;
        if let Some(i) = data.iter().rev().position(|&x| x == GRAM_SEPARATOR) {
            let pos = data.len() - i;
            let pfx = &data[..pos - 1];
            let sfx = &data[pos..];
            Some((Self { data: pfx }, Self { data: sfx }))
        } else {
            None
        }
    }

    /// Pops the last word.
    ///
    /// ```
    /// use tongrams::Gram;
    ///
    /// let words = "abc de f";
    /// let mut gram = Gram::from_str(words);
    ///
    /// let words = gram.split_to_tokens();
    /// assert_eq!(words.len(), 3);
    /// assert_eq!(words[0].raw(), "abc".as_bytes());
    /// assert_eq!(words[1].raw(), "de".as_bytes());
    /// assert_eq!(words[2].raw(), "f".as_bytes());
    /// ```
    pub fn split_to_tokens(&self) -> Vec<Self> {
        self.data
            .split(|&b| b == GRAM_SEPARATOR)
            .map(|data| Self { data })
            .collect()
    }
}

impl<'a> PartialEq for Gram<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'a> fmt::Debug for Gram<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = String::from_utf8(self.data.to_vec()).unwrap();
        f.debug_struct("Gram").field("data", &data).finish()
    }
}
