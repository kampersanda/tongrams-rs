pub mod flate2;
pub mod plain;

use std::io::Read;

use anyhow::Result;

use crate::parser::GramsParser;

pub use crate::loader::flate2::{GramsDeflateFileLoader, GramsGzFileLoader, GramsZlibFileLoader};
pub use crate::loader::plain::{GramsFileLoader, GramsTextLoader};

pub trait GramsLoader<R>
where
    R: Read,
{
    fn parser(&self) -> Result<GramsParser<R>>;
}

// pub enum FileFormats {
//     Plain,
//     Gz,
//     Deflate,
//     Zlib,
// }
