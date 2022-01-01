# `tongrams-rs`: Tons of *N*-grams in Rust

This is a Rust port of [`tongrams`](https://github.com/jermp/tongrams) to index and query large language models in compressed space, in which the data structures are presented in the following papers:

 - Giulio Ermanno Pibiri and Rossano Venturini, [Efficient Data Structures for Massive N-Gram Datasets](https://doi.org/10.1145/3077136.3080798). In *Proceedings of the 40th ACM Conference on Research and Development in Information Retrieval (SIGIR 2017)*, pp. 615-624.
 
 - Giulio Ermanno Pibiri and Rossano Venturini, [Handling Massive N-Gram Datasets Efficiently](https://doi.org/10.1145/3302913). *ACM Transactions on Information Systems (TOIS)*, 37.2 (2019): 1-41.

## Features

 - **Compressed language model.** `tongrams-rs` can store large *N*-gram language models in very compressed space. For example, the word *N*-gram datasets (*N*=1..5) in `test_data` are stored in only 2.6 bytes per gram.
  
 - **Time and memory efficiency.** `tongrams-rs` employs *Elias-Fano Trie*, which cleverly encodes a trie data structure consisting of *N*-grams through *Elias-Fano codes*, enabling fast lookups in compressed space.
  
 - **Pure Rust.** `tongrams-rs` is written only in Rust and can be easily pluged into your Rust codes.

## Input data format

As with the original library, the *N*-gram counts files follow the [Google format](http://storage.googleapis.com/books/ngrams/books/datasetsv2.html).
For the details, please visit [`tongrams`](https://github.com/jermp/tongrams/blob/master/README.md).

## Command line tools

`tools` provides some command line tools to enjoy this library. In the following, the example usages are presented using *N*-gram counts files in `test_data` copied from [`tongrams`](https://github.com/jermp/tongrams).

### 1. Sorting

**NOTE: The current implementation of `sort_grams` consumes a lot amount of memory. When you apply massive datasets, please use the executable `sort_grams` in the original [`tongrams`](https://github.com/jermp/tongrams), which will generate the equivalent dataset.**

To build the trie index, you need to sort your *N*-gram counts files.
First of all, prepare unigram counts files sorted by the counts for making the resulting index smaller, as

```
8761
the	3681
is	1869
a	1778
of	1672
to	1638
and	1202
...
```

By using the unigram file as a vocabulary, the executable `sort_grams` sorts a *N*-gram counts file.

Here, we assume to sort an unsorted bigram counts file, as

```
38900
ways than	1
may come	1
frequent causes	1
way has	1
in which	14
...
```

Let the above unigram and unsorted bigram files in a gzip format be `test_data/1-grams.sorted.gz` and `test_data/2-grams.gz`, respectively.
You can sort the bigram file and write `test_data/2-grams.sorted` with the following command:

```
$ cargo run --release -p tools --bin sort_grams -- -i test_data/2-grams.gz -v test_data/1-grams.sorted.gz -o test_data/2-grams.sorted
WARNING: The current implementation will use a lot of memory.
Loading the vocabulary: "test_data/1-grams.sorted.gz"
Loading the records: "test_data/2-grams.gz"
Sorting the records
Writing the index into "test_data/2-grams.sorted"
```

The resulting `test_data/2-grams.sorted` will be

```
38900
the //	1
the function	94
the if	3
the code	126
the compiler	117
...
```


### 2. Indexing

The executable `index` builds a language model from (sorted) *N*-gram counts files and writes it into a binary file.

For example, the following command builds a language model from *N*-gram counts files placed in `test_data` and writes it into `index.bin`. The specified files must be ordered as 1-gram, 2-gram, and so on.

```
$ cargo run --release -p tools --bin index -- -n 5 -i test_data -o index.bin
Input files: ["test_data/1-grams.sorted.gz", "test_data/2-grams.sorted.gz", "test_data/3-grams.sorted.gz", "test_data/4-grams.sorted.gz", "test_data/5-grams.sorted.gz"]
Counstructing the index...
Elapsed time: 0.190 [sec]
252550 grams are stored.
Writing the index into "index.bin"...
Index size: 659366 bytes (0.629 MiB)
Bytes per gram: 2.611 bytes
```

As the standard output shows, the model file takes only 2.6 bytes per gram.

### 3. Lookup

The executable `lookup` provides a demo to lookup *N*-grams, as follows.

```
$ cargo run --release -p tools --bin lookup -- -i index.bin 
Loading the index from "index.bin"...
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

### 4. Memory statistics

The executable `stats` shows the breakdowns of memory usages for each component.

```
$ cargo run --release -p tools --bin stats -- -i index.bin
Loading the index from "index.bin"...
{"arrays":[{"pointers":5927,"token_ids":55186},{"pointers":19745,"token_ids":92416},{"pointers":25853,"token_ids":107094},{"pointers":28135,"token_ids":111994}],"count_ranks":[{"count_ranks":5350},{"count_ranks":12106},{"count_ranks":13976},{"count_ranks":14582},{"count_ranks":14802}],"counts":[{"count":296},{"count":136},{"count":72},{"count":56},{"count":56}],"vocab":{"data":151560}}
```

## Benchmark

At the directory `bench`, you can measure lookup times using *N*-gram data in `test_data` with the following command:

```
$ RUSTFLAGS="-C target-cpu=native" cargo bench
count_lookup/tongrams/EliasFanoTrieCountLm                                                                            
                        time:   [3.1818 ms 3.1867 ms 3.1936 ms]
```

The reported time is the total elapsed time for looking up 5K random grams.
The above result was actually obtained on my laptop PC (Intel i7, 16GB RAM),
i.e., `EliasFanoTrieCountLm` can look up a gram in 0.64 micro sec on average.

## Todo

- Add fast elias-fano and pertitioned elias-fano
- Add minimal perfect hashing
- Add remapping
- Support probability scores
- Make `sucds::EliasFano` faster
- Use secondary memory for massive files

## Licensing

This library is free software provided under MIT.
