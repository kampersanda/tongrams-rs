# `tongrams-rs`: Tons of *N*-grams in Rust

This is a Rust port of [`tongrams`](https://github.com/jermp/tongrams) to index and query large language models in compressed space, in which the data structures are presented in the following papers:

 - Giulio Ermanno Pibiri and Rossano Venturini, [Efficient Data Structures for Massive N-Gram Datasets](https://doi.org/10.1145/3077136.3080798). In *Proceedings of the 40th ACM Conference on Research and Development in Information Retrieval (SIGIR 2017)*, pp. 615-624.
 - Giulio Ermanno Pibiri and Rossano Venturini, [Handling Massive N-Gram Datasets Efficiently](https://doi.org/10.1145/3302913). *ACM Transactions on Information Systems (TOIS)*, 37.2 (2019): 1-41.

In the current version, `tongrams-rs` implements only the data structure type of `ef_trie_PSEF_ranks_count_lm` whose vocablary is implemented with [yada](https://github.com/takuyaa/yada).

## Input data format

As with the original library, the *N*-gram counts files follow the [Google format](http://storage.googleapis.com/books/ngrams/books/datasetsv2.html).
For the details, please see the [README of tongrams](https://github.com/jermp/tongrams/blob/master/README.md), although the current version does not support `gzip` compressed files.

## Command line tools

`tools` provides some command line tools. In the following, the example usages are described using *N*-gram data in `test_data` copied from [`tongrams`](https://github.com/jermp/tongrams).

### Indexing

The executable `index` builds a language model from *N*-gram counts files and writes it into a file.

For example, the following command builds a language model from *N*-gram counts files placed in `test_data` and writes it into `index.bin`. The specified files must be ordered as 1-gram, 2-gram, and so on.

```
$ cargo run --release -p tools --bin index -- -n 5 -i test_data -o index.bin
Input files: ["test_data/1-grams.sorted", "test_data/2-grams.sorted", "test_data/3-grams.sorted", "test_data/4-grams.sorted", "test_data/5-grams.sorted"]
Counstructing the index...
Elapsed time: 0.144 [sec]
252550 grams are stored.
Writing the index into index.bin...
Index size: 659366 bytes (0.629 MiB)
Bytes per gram: 2.611 bytes
```

As the standard output shows, the model file takes only 2.6 bytes per gram.

### Lookup

The executable `lookup` provides a demo to lookup *N*-grams, as follows.

```
$ cargo run --release -p tools --bin lookup -- -i index.bin 
Loading the index from index.bin...
Performing the lookup...
> take advantage
count = 8
> only 64-bit execution
count = 1
> Elias Fano
Not found
> 
Good bye!
```

### Memory statistics

The executable `stats` shows the breakdowns of memory usages for each component.

```
$ cargo run --release -p tools --bin stats -- -i index.bin
Loading the index from index.bin...
{"arrays":[{"pointers":5927,"token_ids":55186},{"pointers":19745,"token_ids":92416},{"pointers":25853,"token_ids":107094},{"pointers":28135,"token_ids":111994}],"count_ranks":[{"count_ranks":5350},{"count_ranks":12106},{"count_ranks":13976},{"count_ranks":14582},{"count_ranks":14802}],"counts":[{"count":296},{"count":136},{"count":72},{"count":56},{"count":56}],"vocab":{"data":151560}}
```

## Benchmark

At the directory `bench`, you can measure lookup times using *N*-gram data in `test_data` with the following command:

```
$ cargo bench
count_lookup/tongrams/EliasFanoTrieCountLm                                                                            
                        time:   [3.9481 ms 3.9540 ms 3.9605 ms]
```

The reported time is the total elapsed time for looking up 5K random grams.

On my laptop PC (i7, 16GB RAM), the average lookup time was 0.79 micro sec per query, although the original tongram performed lookup in 0.44 micro sec per query.

## Todo

- Support `gzip`
- Add fast elias-fano and pertitioned elias-fano
- Add minimal perfect hashing
- Add remapping
- Support probability scores
- Make `sucds::EliasFano` faster

