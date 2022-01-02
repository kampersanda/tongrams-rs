# `tongrams-rs`: Tons of *N*-grams in Rust

![](https://github.com/kampersanda/tongrams-rs/actions/workflows/rust.yml/badge.svg)
[![Documentation](https://docs.rs/tongrams/badge.svg)](https://docs.rs/tongrams)
[![Crates.io](https://img.shields.io/crates/v/tongrams.svg)](https://crates.io/crates/tongrams)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kampersanda/tongrams-rs/blob/master/LICENSE)

This is a Rust port of [`tongrams`](https://github.com/jermp/tongrams) to index and query large language models in compressed space, in which the data structures are presented in the following papers:

 - Giulio Ermanno Pibiri and Rossano Venturini, [Efficient Data Structures for Massive N-Gram Datasets](https://doi.org/10.1145/3077136.3080798). In *Proceedings of the 40th ACM Conference on Research and Development in Information Retrieval (SIGIR 2017)*, pp. 615-624.
 
 - Giulio Ermanno Pibiri and Rossano Venturini, [Handling Massive N-Gram Datasets Efficiently](https://doi.org/10.1145/3302913). *ACM Transactions on Information Systems (TOIS)*, 37.2 (2019): 1-41.

## What can do

 - Store *N*-gram language models with frequency counts.

 - Look up *N*-grams to get the frequency counts.

## Features

 - **Compressed language model.** `tongrams-rs` can store large *N*-gram language models in very compressed space. For example, the word *N*-gram datasets (*N*=1..5) in `test_data` are stored in only 2.6 bytes per gram.
  
 - **Time and memory efficiency.** `tongrams-rs` employs *Elias-Fano Trie*, which cleverly encodes a trie data structure consisting of *N*-grams through *Elias-Fano codes*, enabling fast lookups in compressed space.
  
 - **Pure Rust.** `tongrams-rs` is written only in Rust and can be easily pluged into your Rust codes.

## Input data format

The file format of *N*-gram counts files is the same as that used in [`tongrams`](https://github.com/jermp/tongrams), a modified [Google format](http://storage.googleapis.com/books/ngrams/books/datasetsv2.html), where

 - one separate file for each distinct value of *N* (order) lists one gram per row,
 - each header row `<number_of_grams>` indicates the number of *N*-grams in the file,
 - tokens in a gram `<gram>` are sparated by a space (e.g., `the same time`), and
 - a gram `<gram>` and the count `<count>` are sparated by a horizontal tab.

```text
<number_of_grams>
<gram1><TAB><count1>
<gram2><TAB><count2>
<gram3><TAB><count3>
...
```

For example,

```text
61516
the // parent	1
the function is	22
the function a	4
the function to	1
the function and	1
...
```

## Command line tools

`tools` provides some command line tools to enjoy this library. In the following, the example usages are presented using *N*-gram counts files in `test_data` copied from [`tongrams`](https://github.com/jermp/tongrams).

### 1. Sorting

To build the trie index, you need to sort your *N*-gram counts files.
First, prepare unigram counts files sorted by the counts for making a resulting index smaller, as

```
$ cat test_data/1-grams.sorted
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

Here, we sort an unsorted bigram counts file, as

```
$ cat test_data/2-grams
38900
ways than	1
may come	1
frequent causes	1
way has	1
in which	14
...
```

You can sort the bigram file (in a gzip format) and write `test_data/2-grams.sorted` with the following command:

```
$ cargo run --release -p tools --bin sort_grams -- -i test_data/2-grams.gz -v test_data/1-grams.sorted.gz -o test_data/2-grams.sorted
Loading the vocabulary: "test_data/1-grams.sorted.gz"
Loading the records: "test_data/2-grams.gz"
Sorting the records
Writing the index into "test_data/2-grams.sorted.gz"
```

The output file format can be specified with `-f`, and the default setting is `.gz`. The resulting file will be

```
$ cat test_data/2-grams.sorted
38900
the //	1
the function	94
the if	3
the code	126
the compiler	117
...
```


### 2. Indexing

The executable `index` builds a language model from (sorted) *N*-gram counts files, named `<order>-grams.sorted.gz`, and writes it into a binary file. The input file format can be specified with `-f`, and the default setting is `.gz`.

For example, the following command builds a language model from *N*-gram counts files (*N*=1..5) placed in directory `test_data` and writes it into `index.bin`.

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

## Licensing

This library is free software provided under MIT.
