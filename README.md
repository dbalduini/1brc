# The One Billion Row Challenge

https://github.com/gunnarmorling/1brc/tree/main

## Generating data

```
cd C:\Dev\Github\1brc
```

**1M rows**

```
java --class-path target/average-1.0.0-SNAPSHOT.jar dev.morling.onebrc.CreateMeasurements 1000000
```

37ms

**10M rows**

```
java --class-path target/average-1.0.0-SNAPSHOT.jar dev.morling.onebrc.CreateMeasurements 10000000
```

200ms

**100M rows**
```
java --class-path target/average-1.0.0-SNAPSHOT.jar dev.morling.onebrc.CreateMeasurements 100000000
```

1.75s


## Benchmark

- [Profiling Tools](https://github.com/rust-unofficial/awesome-rust?tab=readme-ov-file#profiling)
- [hyperfine](https://github.com/sharkdp/hyperfine)

```
hyperfine --warmup 3 '.\target\debug\lbrc.exe .\data\1M.csv'
```

## Profiling

Using `perf` and `flamegraph` on WSL2.

https://www.brendangregg.com/FlameGraphs/cpuflamegraphs.html

```
perf target/debug/lbrc ./data/1M.csv
```


```
fg -- target/debug/lbrc ./data/1M.csv
```

## Optimizing

It's most helpful to interpret the numbers on a per-row basis (dividing everything by 1 million).

### Naive approach

```
 612988712      branches:u
   3783128      branch-misses:u           #    0.62% of all branches
    466196      cache-references:u
    198627      cache-misses:u            #   42.606 % of all cache refs
1725020646      cycles:u
4288327760      instructions:u            #    2.49  insn per cycle

0.541212437 seconds time elapsed
0.387241000 seconds user
0.030571000 seconds sys
```

4288327760 / 1000000
~ 4288 instructions per row

### Improved hash map


```
209062394      branches:u
  4673514      branch-misses:u           #    2.24% of all branches
   439691      cache-references:u
   181798      cache-misses:u            #   41.347 % of all cache refs
386597003      cycles:u
945244352      instructions:u            #    2.45  insn per cycle

0.163670339 seconds time elapsed
0.092252000 seconds user
0.009225000 seconds sys
```

945244352 / 1000000
~ 945 instructions per row



## DuckDB Benchmark

```
❯ hyperfine --warmup 5 'duckdb -no-stdin -init .\query.sql'

Benchmark 1: duckdb -no-stdin -init .\query.sql
  Time (mean ± σ):     128.0 ms ±   3.6 ms    [User: 266.4 ms, System: 17.0 ms]
  Range (min … max):   115.3 ms … 133.2 ms    20 runs
```

Versus multithread version

```
❯ hyperfine --warmup 5 '.\target\release\lbrc.exe .\data\1M.csv'

Benchmark 1: .\target\release\lbrc.exe .\data\1M.csv
  Time (mean ± σ):      37.5 ms ±   2.8 ms    [User: 13.9 ms, System: 2.0 ms]
  Range (min … max):    34.0 ms …  47.6 ms    54 runs
```

### Chunk Reader optimization

```
Tokyo;35.6897
Jakarta;-6.1750
Delhi;28.6100
```

Each worker receives a view of the full file, starting from offset until `offset` + `chunk_size`.

Chunk 1:
```
Tokyo;35.6897
Jakarta;
```

Chunk 2:
```
-6.1750
Delhi;28.6100
```

The idea is to expand Chunk 1 until next `\n` and cut Chunk 2 to start at `\nDelhi`.
- To expand, the chunk must ends with `\n`.
- To cut, the chunk must start with an Upper Letter.

Chunk 1:
```
Tokyo;35.6897
Jakarta;-6.1750
```

Chunk 2:
```
Delhi;28.6100
```

# Results

Processed 1 Billion Rows File (~16gb) in ~25 seconds.


# Resources

## Challenges 

- https://questdb.io/blog/billion-row-challenge-step-by-step/
- https://aminediro.com/posts/billion_row/?utm_source=pocket_saves#

