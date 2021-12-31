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

```
{"arrays":[{"count_ranks":5350,"pointers":82,"sampled_ids":82,"token_ids":82},{"count_ranks":12106,"pointers":5927,"sampled_ids":14910,"token_ids":55186},{"count_ranks":13976,"pointers":19745,"sampled_ids":61866,"token_ids":92416},{"count_ranks":14582,"pointers":25853,"sampled_ids":95656,"token_ids":107094},{"count_ranks":14802,"pointers":28135,"sampled_ids":108038,"token_ids":111994}],"counts":[{"count":296},{"count":136},{"count":72},{"count":56},{"count":56}],"vocab":{"data":151560}}
```