# tongrams-rs
Rust port of tongrams

```
$ cargo run --release  -p index -- -i test_data/1-grams.sorted test_data/2-grams.sorted test_data/3-grams.sorted test_data/4-grams.sorted test_data/5-grams.sorted -o index.out
Counstructing the index...
Elapsed time: 0.146 [sec]
Writing the index into index.out...
yada size = 151552
Index size: 940074 bytes (0.897 MiB)
```

```
$ cargo run --release -p lookup -- -i index.out
```