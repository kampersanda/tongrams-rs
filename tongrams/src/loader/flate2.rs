use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use flate2::read::{DeflateDecoder, GzDecoder, ZlibDecoder};

use crate::loader::GramsLoader;
use crate::parser::GramsParser;

pub struct GramsGzFileLoader {
    filepath: PathBuf,
}

impl GramsGzFileLoader {
    pub fn new<P>(filepath: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            filepath: PathBuf::from(filepath.as_ref()),
        }
    }
}

impl GramsLoader<GzDecoder<File>> for GramsGzFileLoader {
    fn parser(&self) -> Result<GramsParser<GzDecoder<File>>> {
        let reader = GzDecoder::new(File::open(&self.filepath)?);
        GramsParser::new(BufReader::new(reader))
    }
}

pub struct GramsDeflateFileLoader {
    filepath: PathBuf,
}

impl GramsDeflateFileLoader {
    pub fn new<P>(filepath: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            filepath: PathBuf::from(filepath.as_ref()),
        }
    }
}

impl GramsLoader<DeflateDecoder<File>> for GramsDeflateFileLoader {
    fn parser(&self) -> Result<GramsParser<DeflateDecoder<File>>> {
        let reader = DeflateDecoder::new(File::open(&self.filepath)?);
        GramsParser::new(BufReader::new(reader))
    }
}

pub struct GramsZlibFileLoader {
    filepath: PathBuf,
}

impl GramsZlibFileLoader {
    pub fn new<P>(filepath: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            filepath: PathBuf::from(filepath.as_ref()),
        }
    }
}

impl GramsLoader<ZlibDecoder<File>> for GramsZlibFileLoader {
    fn parser(&self) -> Result<GramsParser<ZlibDecoder<File>>> {
        let reader = ZlibDecoder::new(File::open(&self.filepath)?);
        GramsParser::new(BufReader::new(reader))
    }
}
