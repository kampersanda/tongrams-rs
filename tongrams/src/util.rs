use std::io::Read;
use std::path::Path;

use anyhow::Result;

use crate::loader::{
    GramsDeflateFileLoader, GramsFileLoader, GramsGzFileLoader, GramsLoader, GramsZlibFileLoader,
};
use crate::vocabulary::{DoubleArrayVocabulary, Vocabulary};
use crate::{Gram, GramsFileFormats, Record};

/// Loads all of [`Record`] from a file.
///
/// # Arguments
///
///  - `filepath`: *N*-gram counts file.
///  - `fmt`: File format.
pub fn load_records_from_file<P>(filepath: P, fmt: GramsFileFormats) -> Result<Vec<Record>>
where
    P: AsRef<Path>,
{
    match fmt {
        GramsFileFormats::Plain => {
            let loader: Box<dyn GramsLoader<_>> = Box::new(GramsFileLoader::new(filepath));
            load_records(loader)
        }
        GramsFileFormats::Gzip => {
            let loader: Box<dyn GramsLoader<_>> = Box::new(GramsGzFileLoader::new(filepath));
            load_records(loader)
        }
        GramsFileFormats::Deflate => {
            let loader: Box<dyn GramsLoader<_>> = Box::new(GramsDeflateFileLoader::new(filepath));
            load_records(loader)
        }
        GramsFileFormats::Zlib => {
            let loader: Box<dyn GramsLoader<_>> = Box::new(GramsZlibFileLoader::new(filepath));
            load_records(loader)
        }
    }
}

/// Loads all of [`Record`] from a gzipped gram-count file.
fn load_records<R: Read>(loader: Box<dyn GramsLoader<R>>) -> Result<Vec<Record>>
where
    R: Read,
{
    let gp = loader.parser()?;
    let mut records = Vec::new();
    for rec in gp {
        let rec = rec?;
        records.push(rec);
    }
    Ok(records)
}

/// Builds [`DoubleArrayVocabulary`] from a file.
///
/// # Arguments
///
///  - `filepath`: *N*-gram counts file.
///  - `fmt`: File format.
pub fn build_vocabulary_from_file<P>(
    filepath: P,
    fmt: GramsFileFormats,
) -> Result<DoubleArrayVocabulary>
where
    P: AsRef<Path>,
{
    let records = load_records_from_file(filepath, fmt)?;
    let grams: Vec<Gram> = records.iter().map(|r| r.gram()).collect();
    let vocab = *DoubleArrayVocabulary::build(&grams)?;
    Ok(vocab)
}

/// Gets the file extension of *N*-gram counts file.
pub fn get_format_extension(fmt: GramsFileFormats) -> Option<String> {
    match fmt {
        GramsFileFormats::Plain => None,
        GramsFileFormats::Gzip => Some("gz".to_string()),
        GramsFileFormats::Deflate => Some("dfl".to_string()),
        GramsFileFormats::Zlib => Some("zlib".to_string()),
    }
}
