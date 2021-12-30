use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::parser::GramsParser;

pub trait GramsLoader<R>
where
    R: Read,
{
    fn parser(&self) -> Result<GramsParser<R>>;
}

pub struct GramsFileLoader {
    filename: String,
}

impl GramsFileLoader {
    pub const fn new(filename: String) -> Self {
        Self { filename }
    }
}

impl GramsLoader<File> for GramsFileLoader {
    fn parser(&self) -> Result<GramsParser<File>> {
        let reader = BufReader::new(File::open(&self.filename)?);
        GramsParser::new(reader)
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
