use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;

const TEST_FILENAMES: [&str; 5] = [
    "../test_data/1-grams.sorted.gz",
    "../test_data/2-grams.sorted.gz",
    "../test_data/3-grams.sorted.gz",
    "../test_data/4-grams.sorted.gz",
    "../test_data/5-grams.sorted.gz",
];

const TEST_QUERIES: &str = "../test_data/queries.random.5K.txt";

use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion, SamplingMode,
};

const SAMPLE_SIZE: usize = 30;
const WARM_UP_TIME: Duration = Duration::from_secs(5);
const MEASURE_TIME: Duration = Duration::from_secs(10);

fn load_queries() -> Vec<String> {
    let file = File::open(TEST_QUERIES).expect("No such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn criterion_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("count_lookup");
    group.sample_size(SAMPLE_SIZE);
    group.warm_up_time(WARM_UP_TIME);
    group.measurement_time(MEASURE_TIME);
    group.sampling_mode(SamplingMode::Flat);

    let gram_files: Vec<PathBuf> = TEST_FILENAMES
        .iter()
        .map(|f| PathBuf::from_str(f).unwrap())
        .collect();

    let queries = load_queries();
    let qgrams: Vec<tongrams::Gram> = queries
        .iter()
        .map(|q| tongrams::Gram::from_str(q))
        .collect();

    perform_lookup(&mut group, &gram_files, &qgrams);
}

fn perform_lookup(
    group: &mut BenchmarkGroup<WallTime>,
    gram_files: &[PathBuf],
    queries: &[tongrams::Gram],
) {
    let lm = tongrams::EliasFanoTrieCountLm::from_gz_files(gram_files).unwrap();
    group.bench_function("tongrams/EliasFanoTrieCountLm", |b| {
        let mut lookuper = lm.lookuper();
        b.iter(|| {
            let mut sum = 0;
            for &q in queries {
                sum += lookuper.with_gram(q).unwrap_or(1);
            }
            if sum == 0 {
                panic!();
            }
        });
    });
}

criterion_group!(benches, criterion_lookup,);

criterion_main!(benches);
