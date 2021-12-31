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
{
   "arrays":[
      {
         "count_ranks":5350,
         "pointers":82,
         "sampled_ids":82,
         "token_ids":82
      },
      {
         "count_ranks":12106,
         "pointers":5927,
         "sampled_ids":14910,
         "token_ids":55186
      },
      {
         "count_ranks":13976,
         "pointers":19745,
         "sampled_ids":61866,
         "token_ids":92416
      },
      {
         "count_ranks":14582,
         "pointers":25853,
         "sampled_ids":95656,
         "token_ids":107094
      },
      {
         "count_ranks":14802,
         "pointers":28135,
         "sampled_ids":108038,
         "token_ids":111994
      }
   ],
   "counts":[
      {
         "count":296
      },
      {
         "count":136
      },
      {
         "count":72
      },
      {
         "count":56
      },
      {
         "count":56
      }
   ],
   "vocab":{
      "data":151560
   }
}
```

`sampled_ids` seems to be overhead compared to the original...

```
$ wc ef_trie.count.PSEF.bin
   2443  12589 587333 ef_trie.count.PSEF.bin

% ./print_stats ef_trie.count.PSEF.bin
==== tongrams binary format ====
library version: 1.0
data structure type: ef_trie
remapping order: 0
ranks type: PSEF
value type: count
================================
2021-12-31 17:59:57: Loading data structure
2021-12-31 17:59:57: ========= TRIE_COUNT_LM statistics =========
1-grams bytes:
        ranks: 6133 (0.700034 per gram)
        pointers: 5213 (0.595023 per gram)
2-grams bytes:
        grams: 55881 (1.43653 per gram)
        ranks: 14613 (0.375656 per gram)
        pointers: 15385 (0.395501 per gram)
3-grams bytes:
        grams: 92471 (1.5032 per gram)
        ranks: 15045 (0.244571 per gram)
        pointers: 20887 (0.339538 per gram)
4-grams bytes:
        grams: 107093 (1.52585 per gram)
        ranks: 15047 (0.214387 per gram)
        pointers: 22957 (0.327088 per gram)
5-grams bytes:
        grams: 111993 (1.53023 per gram)
        ranks: 15059 (0.205761 per gram)
order: 5
num. of grams: 252550
tot. bytes: 587333
bytes per gram: 2.32561
vocabulary data bytes: 85448
        (does NOT include hash function bytes)
unique values bytes: 576
tot. grams bytes: 367519 (62.5742%)
        per gram: 1.45523
tot. ranks bytes: 65897 (11.2197%)
        per gram: 0.260927
tot. pointers bytes: 64531 (10.9871%)
        per gram: 0.255518
number of unique values:
        1-grams: 176
        2-grams: 90
        3-grams: 39
        4-grams: 26
        5-grams: 22

trie level statistics:
        1-grams:
        num grams: 8761
        num ptrs.: 8762
        non-empty ranges: 8761
        density: 99.9886%
        min range: 1
        max range: 873
        avg range: 4.44013
        top-5 ranges: 873 606 540 507 507 

        2-grams:
        num grams: 38900
        universe: 17600063
        avg gap: 452.444
        num ptrs.: 38901
        non-empty ranges: 38899
        density: 99.9949%
        min range: 1
        max range: 206
        avg range: 1.58143
        top-5 ranges: 206 188 160 117 95 
        H_0 = 1.3725 bytes
        H_1 = 0.522306 bytes

        3-grams:
        num grams: 61516
        universe: 45461872
        avg gap: 739.025
        num ptrs.: 61517
        non-empty ranges: 61515
        density: 99.9967%
        min range: 1
        max range: 48
        avg range: 1.14096
        top-5 ranges: 48 27 24 23 23 
        H_2 = 0.159448 bytes

        4-grams:
        num grams: 70186
        universe: 58729338
        avg gap: 836.767
        num ptrs.: 70187
        non-empty ranges: 70185
        density: 99.9972%
        min range: 1
        max range: 14
        avg range: 1.04277
        top-5 ranges: 14 14 12 12 12 

        5-grams:
        num grams: 73187
        universe: 62554491
        avg gap: 854.721
```