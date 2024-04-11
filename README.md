# The One Billion Row Challenge

https://github.com/gunnarmorling/1brc/tree/main

## Test data

```
cd C:\Dev\Github\1brc
```

1M rows

```
java --class-path target/average-1.0.0-SNAPSHOT.jar dev.morling.onebrc.CreateMeasurements 1000000
```

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

### Naive approach

It's most helpful to interpret the numbers on a per-row basis (dividing everything by 1 million).

```
 Performance counter stats for 'target/debug/lbrc ./data/1M.csv':

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

