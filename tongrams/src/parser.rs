use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Read};

use crate::Record;
use crate::GRAM_COUNT_SEPARATOR;

pub struct GramsParser<R>
where
    R: Read,
{
    reader: BufReader<R>,
    num_grams: usize,
    count: usize,
}

impl<R> GramsParser<R>
where
    R: Read,
{
    pub fn new(mut reader: BufReader<R>) -> Result<Self> {
        let num_grams = {
            let mut header = String::new();
            reader.read_line(&mut header)?;
            header.trim().parse()?
        };
        Ok(Self {
            reader,
            num_grams,
            count: 0,
        })
    }

    pub fn num_grams(&self) -> usize {
        self.num_grams
    }
}

impl<R> Iterator for GramsParser<R>
where
    R: Read,
{
    type Item = Result<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count > self.num_grams {
            return None;
        }

        let mut buffer = String::new();
        self.reader.read_line(&mut buffer).ok()?;

        let items: Vec<&str> = buffer.trim().split(GRAM_COUNT_SEPARATOR as char).collect();
        if items.len() != 2 {
            return Some(Err(anyhow!("Invalid line")));
        }

        let gram = items[0].to_string();
        if let Ok(count) = items[1].parse() {
            Some(Ok(Record::new(gram, count)))
        } else {
            Some(Err(anyhow!("Parse error")))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.num_grams, Some(self.num_grams))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRAMS_1: &'static str = "4
A\t10
B\t7
C\t4
D\t1
";

    const GRAMS_2: &'static str = "4
A A\t1
A C\t2
B B\t3
D C\t1
";

    const GRAMS_3: &'static str = "3
A A C\t2
B B C\t1
D D D\t1
";

    #[test]
    fn test_grams_1() {
        let mut gp = GramsParser::new(BufReader::new(GRAMS_1.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 4);
        for (gram, count) in [("A", 10), ("B", 7), ("C", 4), ("D", 1)] {
            let gram = gram.to_string();
            assert_eq!(gp.next().unwrap().unwrap(), Record::new(gram, count));
        }
        assert!(gp.next().is_none());
    }

    #[test]
    fn test_grams_2() {
        let mut gp = GramsParser::new(BufReader::new(GRAMS_2.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 4);
        for (gram, count) in [("A A", 1), ("A C", 2), ("B B", 3), ("D C", 1)] {
            let gram = gram.to_string();
            assert_eq!(gp.next().unwrap().unwrap(), Record::new(gram, count));
        }
        assert!(gp.next().is_none());
    }

    #[test]
    fn test_grams_3() {
        let mut gp = GramsParser::new(BufReader::new(GRAMS_3.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 3);
        for (gram, count) in [("A A C", 2), ("B B C", 1), ("D D D", 1)] {
            let gram = gram.to_string();
            assert_eq!(gp.next().unwrap().unwrap(), Record::new(gram, count));
        }
        assert!(gp.next().is_none());
    }
}
