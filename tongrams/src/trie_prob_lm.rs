mod builder;

/// Elias-Fano trie for indexing *N*-grams with their frequency counts.
#[derive(Default, Debug)]
pub struct TrieProbLm<T, V> {
    vocab: V,
    arrays: Vec<T>,
    probs: Vec<Vec<f32>>,    // TODO: Quantize
    backoffs: Vec<Vec<f32>>, // TODO: Quantize
}
