# tongrams-rs
Rust port of tongrams

```
$ cargo run --release  -p index -- -i test_data/1-grams.sorted test_data/2-grams.sorted test_data/3-grams.sorted test_data/4-grams.sorted test_data/5-grams.sorted -o index.out
```

```
$ cargo run --release -p lookup -- -i index.out
```