use std::path::Path;

use anyhow::Result;

use crate::loader::{GramsGzFileLoader, GramsLoader};
use crate::vocabulary::{DoubleArrayVocabulary, Vocabulary};
use crate::{Gram, Record};

/// Loads all of [`Record`] from a gzipped gram-count file.
pub fn load_records_from_gz<P>(filepath: P) -> Result<Vec<Record>>
where
    P: AsRef<Path>,
{
    let loader = GramsGzFileLoader::new(filepath);
    let gp = loader.parser()?;
    let mut records = Vec::new();
    for rec in gp {
        let rec = rec?;
        records.push(rec);
    }
    Ok(records)
}

/// Builds [`DoubleArrayVocabulary`] from a gzipped gram-count file.
pub fn build_vocabulary_from_gz<P>(filepath: P) -> Result<DoubleArrayVocabulary>
where
    P: AsRef<Path>,
{
    let records = load_records_from_gz(filepath)?;
    let grams: Vec<Gram> = records.iter().map(|r| r.gram()).collect();
    let vocab = *DoubleArrayVocabulary::build(&grams)?;
    Ok(vocab)
}
