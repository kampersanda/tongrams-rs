mod flate2;
mod plain;

use std::io::Read;

use anyhow::Result;

use crate::parser::GramsParser;

pub use crate::loader::flate2::{GramsDeflateFileLoader, GramsGzFileLoader, GramsZlibFileLoader};
pub use crate::loader::plain::{GramsFileLoader, GramsTextLoader};

/// Loader for a *N*-gram counts file.
pub trait GramsLoader<R>
where
    R: Read,
{
    /// Creates [`GramsParser`].
    fn parser(&self) -> Result<GramsParser<R>>;
}

/// File formats supported.
pub enum GramsFileFormats {
    Plain,
    Gzip,
    Deflate,
    Zlib,
}
