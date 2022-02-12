mod builder;

use anyhow::Result;

use crate::loader::{GramsLoader, GramsTextLoader};
use crate::trie_array::TrieArray;
use crate::vocabulary::Vocabulary;

pub use builder::TrieProbLmBuilder;

/// Elias-Fano trie for indexing *N*-grams with their frequency counts.
#[derive(Default, Debug)]
pub struct TrieProbLm<T, V> {
    vocab: V,
    arrays: Vec<T>,
    probs: Vec<Vec<f32>>,    // TODO: Quantize
    backoffs: Vec<Vec<f32>>, // TODO: Quantize
}

impl<T, V> TrieProbLm<T, V>
where
    T: TrieArray,
    V: Vocabulary,
{
    /// Builds the index from *N*-gram models of raw texts (for debug).
    #[doc(hidden)]
    pub fn from_texts(texts: Vec<&'static str>) -> Result<Self> {
        let mut loaders = Vec::with_capacity(texts.len());
        for text in texts {
            let loader: Box<dyn GramsLoader<_>> = Box::new(GramsTextLoader::new(text.as_bytes()));
            loaders.push(loader);
        }
        TrieProbLmBuilder::new(loaders)?.build()
    }
}
