use anyhow::{anyhow, Result};
use std::io::{BufRead, BufReader, Read};

use crate::GRAM_COUNT_SEPARATOR;
use crate::{CountRecord, ProbRecord};

/// Parser for a *N*-gram file of counts.
/// It assumes the input format of the
/// [Google format](http://storage.googleapis.com/books/ngrams/books/datasetsv2.html).
pub struct GramsParser<R> {
    reader: BufReader<R>,
    num_grams: usize,
    count: usize,
}

impl<R> GramsParser<R>
where
    R: Read,
{
    /// Creates [`GramsParser`] from `BufReader` of a *N*-gram file of counts.
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

    /// Gets the number of input grams.
    pub fn num_grams(&self) -> usize {
        self.num_grams
    }

    pub fn next_count_record(&mut self) -> Option<Result<CountRecord>> {
        self.count += 1;
        if self.count > self.num_grams {
            return None;
        }

        let mut buffer = String::new();
        self.reader.read_line(&mut buffer).ok()?;

        let items: Vec<&str> = buffer
            .trim_end()
            .split(GRAM_COUNT_SEPARATOR as char)
            .collect();
        if items.len() != 2 {
            return Some(Err(anyhow!("Invalid line, {:?}", items)));
        }

        let gram = items[0].to_string();
        if let Ok(count) = items[1].parse() {
            Some(Ok(CountRecord::new(gram, count)))
        } else {
            Some(Err(anyhow!("Parse error, {:?}", items)))
        }
    }

    pub fn next_prob_record(&mut self) -> Option<Result<ProbRecord>> {
        self.count += 1;
        if self.count > self.num_grams {
            return None;
        }

        let mut buffer = String::new();
        self.reader.read_line(&mut buffer).ok()?;

        let items: Vec<&str> = buffer
            .trim_end()
            .split(GRAM_COUNT_SEPARATOR as char)
            .collect();
        if items.len() != 2 && items.len() != 3 {
            return Some(Err(anyhow!("Invalid line, {:?}", items)));
        }

        let gram = items[1].to_string();
        if let Ok(prob) = items[0].parse() {
            let prob = if prob > 0.0 {
                eprintln!(
                    "Warning: positive log10 probability detected. This will be mapped to 0."
                );
                0.0
            } else {
                prob
            };
            if let Some(x) = items.get(2) {
                if let Ok(backoff) = x.parse() {
                    Some(Ok(ProbRecord::new(gram, prob, backoff)))
                } else {
                    Some(Err(anyhow!("Parse error, {:?}", items)))
                }
            } else {
                Some(Ok(ProbRecord::new(gram, prob, 0.0)))
            }
        } else {
            Some(Err(anyhow!("Parse error, {:?}", items)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COUNT_GRAMS_1: &'static str = "4
A\t10
B\t7
C\t4
D\t1
";

    const COUNT_GRAMS_2: &'static str = "4
A A\t1
A C\t2
B B\t3
D C\t1
";

    const COUNT_GRAMS_3: &'static str = "3
A A C\t2
B B C\t1
D D D\t1
";

    const PROB_GRAMS_1: &'static str = "4
-1.83\tA\t-0.74
-2.01\tB\t-0.69
-2.22\tC
-1.91\tD\t-0.62
";

    const PROB_GRAMS_2: &'static str = "4
-1.43\tA A\t-0.33
-0.59\tA C\t-0.43
-1.03\tB B\t-0.32
-1.08\tD C
";

    const PROB_GRAMS_3: &'static str = "3
-1.12\tA A C
-0.53\tB B C
-0.98\tD D D
";

    #[test]
    fn test_count_grams_1() {
        let mut gp = GramsParser::new(BufReader::new(COUNT_GRAMS_1.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 4);
        for (gram, count) in [("A", 10), ("B", 7), ("C", 4), ("D", 1)] {
            let gram = gram.to_string();
            assert_eq!(
                gp.next_count_record().unwrap().unwrap(),
                CountRecord::new(gram, count)
            );
        }
        assert!(gp.next_count_record().is_none());
    }

    #[test]
    fn test_count_grams_2() {
        let mut gp = GramsParser::new(BufReader::new(COUNT_GRAMS_2.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 4);
        for (gram, count) in [("A A", 1), ("A C", 2), ("B B", 3), ("D C", 1)] {
            let gram = gram.to_string();
            assert_eq!(
                gp.next_count_record().unwrap().unwrap(),
                CountRecord::new(gram, count)
            );
        }
        assert!(gp.next_count_record().is_none());
    }

    #[test]
    fn test_count_grams_3() {
        let mut gp = GramsParser::new(BufReader::new(COUNT_GRAMS_3.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 3);
        for (gram, count) in [("A A C", 2), ("B B C", 1), ("D D D", 1)] {
            let gram = gram.to_string();
            assert_eq!(
                gp.next_count_record().unwrap().unwrap(),
                CountRecord::new(gram, count)
            );
        }
        assert!(gp.next_count_record().is_none());
    }

    #[test]
    fn test_prob_grams_1() {
        let mut gp = GramsParser::new(BufReader::new(PROB_GRAMS_1.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 4);
        for (gram, prob, backoff) in [
            ("A", -1.83, -0.74),
            ("B", -2.01, -0.69),
            ("C", -2.22, 0.0),
            ("D", -1.91, -0.62),
        ] {
            let gram = gram.to_string();
            assert_eq!(
                gp.next_prob_record().unwrap().unwrap(),
                ProbRecord::new(gram, prob, backoff)
            );
        }
        assert!(gp.next_prob_record().is_none());
    }

    #[test]
    fn test_prob_grams_2() {
        let mut gp = GramsParser::new(BufReader::new(PROB_GRAMS_2.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 4);
        for (gram, prob, backoff) in [
            ("A A", -1.43, -0.33),
            ("A C", -0.59, -0.43),
            ("B B", -1.03, -0.32),
            ("D C", -1.08, 0.0),
        ] {
            let gram = gram.to_string();
            assert_eq!(
                gp.next_prob_record().unwrap().unwrap(),
                ProbRecord::new(gram, prob, backoff)
            );
        }
        assert!(gp.next_prob_record().is_none());
    }

    #[test]
    fn test_prob_grams_3() {
        let mut gp = GramsParser::new(BufReader::new(PROB_GRAMS_3.as_bytes())).unwrap();
        assert_eq!(gp.num_grams(), 3);
        for (gram, prob, backoff) in [
            ("A A C", -1.12, 0.0),
            ("B B C", -0.53, 0.0),
            ("D D D", -0.98, 0.0),
        ] {
            let gram = gram.to_string();
            assert_eq!(
                gp.next_prob_record().unwrap().unwrap(),
                ProbRecord::new(gram, prob, backoff)
            );
        }
        assert!(gp.next_prob_record().is_none());
    }
}
