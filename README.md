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
