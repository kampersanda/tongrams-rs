use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use anyhow::Result;
use flate2::read::GzDecoder;

use crate::parser::GramsParser;

pub trait GramsLoader<R>
where
    R: Read,
{
    fn parser(&self) -> Result<GramsParser<R>>;
}

pub struct GramsFileLoader {
    filepath: PathBuf,
}

impl GramsFileLoader {
    pub const fn new(filepath: PathBuf) -> Self {
        Self { filepath }
    }
}

impl GramsLoader<File> for GramsFileLoader {
    fn parser(&self) -> Result<GramsParser<File>> {
        let reader = BufReader::new(File::open(&self.filepath)?);
        GramsParser::new(reader)
    }
}

pub struct GramsGzFileLoader {
    filepath: PathBuf,
}

impl GramsGzFileLoader {
    pub const fn new(filepath: PathBuf) -> Self {
        Self { filepath }
    }
}

impl GramsLoader<GzDecoder<File>> for GramsGzFileLoader {
    fn parser(&self) -> Result<GramsParser<GzDecoder<File>>> {
        let reader = GzDecoder::new(File::open(&self.filepath)?);
        GramsParser::new(BufReader::new(reader))
    }
}

pub struct GramsTextLoader<'a> {
    text: &'a [u8],
}

impl<'a> GramsTextLoader<'a> {
    pub const fn new(text: &'a [u8]) -> Self {
        Self { text }
    }
}

impl<'a> GramsLoader<&'a [u8]> for GramsTextLoader<'a> {
    fn parser(&self) -> Result<GramsParser<&'a [u8]>> {
        let reader = BufReader::new(self.text);
        GramsParser::new(reader)
    }
}
