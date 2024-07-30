# Router

Proof of concept.

## Benchmarks + Tests

### `matchit` benchmarks

```
Simple Routing/mine
  time:   [188.97 ns 189.30 ns 189.71 ns]

Simple Routing/matchit
  time:   [188.79 ns 189.34 ns 189.95 ns]

Simple Routing/gonzales
  time:   [154.03 ns 154.37 ns 154.77 ns]

simple_divan            fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ matches                            │               │               │               │         │
   ├─ gonzales_matches  155.9 ns      │ 168.9 ns      │ 158.5 ns      │ 158.5 ns      │ 100     │ 3200
   ├─ matchit_matches   192.4 ns      │ 230.1 ns      │ 198.9 ns      │ 200.5 ns      │ 100     │ 3200
   ╰─ mine_matches      180.6 ns      │ 201.4 ns      │ 185.8 ns      │ 187.5 ns      │ 100     │ 3200
```

### `path_tree` benchmarks

```
routers/mine
  time:   [20.307 µs 20.354 µs 20.409 µs]

routers/actix_router
  time:   [4.2535 ms 4.2706 ms 4.2896 ms]

routers/ntex_router
  time:   [185.48 µs 185.97 µs 186.53 µs]

routers/path_table
  time:   [52.066 µs 52.238 µs 52.431 µs]

routers/path_tree
  time:   [30.845 µs 30.946 µs 31.046 µs]

routers/matchit
  time:   [24.996 µs 25.059 µs 25.130 µs]

routers/route_recognizer
  time:   [443.47 µs 445.38 µs 448.71 µs]

path_tree_divan                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ matches                                    │               │               │               │         │
   ├─ actix_router_matches      4.098 ms      │ 8.359 ms      │ 4.123 ms      │ 4.246 ms      │ 100     │ 100
   │                            alloc:        │               │               │               │         │
   │                              41974       │ 0             │ 41974         │ 41554         │         │
   │                              2.721 MB    │ 0 B           │ 2.721 MB      │ 2.694 MB      │         │
   │                            dealloc:      │               │               │               │         │
   │                              41974       │ 0             │ 41974         │ 41554         │         │
   │                              2.721 MB    │ 0 B           │ 2.721 MB      │ 2.694 MB      │         │
   ├─ matchit_matches           24.99 µs      │ 41.45 µs      │ 26.56 µs      │ 27.4 µs       │ 100     │ 100
   │                            alloc:        │               │               │               │         │
   │                              323         │ 323           │ 323           │ 323           │         │
   │                              41.34 KB    │ 41.34 KB      │ 41.34 KB      │ 41.34 KB      │         │
   │                            dealloc:      │               │               │               │         │
   │                              323         │ 323           │ 323           │ 323           │         │
   │                              41.34 KB    │ 41.34 KB      │ 41.34 KB      │ 41.34 KB      │         │
   ├─ mine_matches              20.49 µs      │ 29.66 µs      │ 20.62 µs      │ 20.8 µs       │ 100     │ 100
   ├─ ntex_router_matches       184.4 µs      │ 561.8 µs      │ 188.2 µs      │ 196.9 µs      │ 100     │ 100
   │                            alloc:        │               │               │               │         │
   │                              799         │ 3935          │ 799           │ 830.3         │         │
   │                              59.1 KB     │ 397.1 KB      │ 59.1 KB       │ 62.48 KB      │         │
   │                            dealloc:      │               │               │               │         │
   │                              799         │ 1023          │ 799           │ 801.2         │         │
   │                              59.1 KB     │ 61.12 KB      │ 59.1 KB       │ 59.12 KB      │         │
   │                            grow:         │               │               │               │         │
   │                              0           │ 448           │ 0             │ 4.48          │         │
   │                              0 B         │ 86.01 KB      │ 0 B           │ 860.1 B       │         │
   ├─ path_table_matches        52.29 µs      │ 75.04 µs      │ 52.7 µs       │ 53.56 µs      │ 100     │ 100
   │                            alloc:        │               │               │               │         │
   │                              530         │ 530           │ 530           │ 530           │         │
   │                              55.42 KB    │ 55.42 KB      │ 55.42 KB      │ 55.42 KB      │         │
   │                            dealloc:      │               │               │               │         │
   │                              530         │ 530           │ 530           │ 530           │         │
   │                              55.42 KB    │ 55.42 KB      │ 55.42 KB      │ 55.42 KB      │         │
   ├─ path_tree_matches         32.54 µs      │ 49.66 µs      │ 32.87 µs      │ 33.61 µs      │ 100     │ 100
   ╰─ route_recognizer_matches  441.4 µs      │ 479.2 µs      │ 444.4 µs      │ 447.6 µs      │ 100     │ 100
                                alloc:        │               │               │               │         │
                                  13677       │ 13677         │ 13677         │ 13677         │         │
                                  905.5 KB    │ 905.5 KB      │ 905.5 KB      │ 905.5 KB      │         │
                                dealloc:      │               │               │               │         │
                                  13677       │ 13677         │ 13677         │ 13677         │         │
                                  980.8 KB    │ 980.8 KB      │ 980.8 KB      │ 980.8 KB      │         │
                                grow:         │               │               │               │         │
                                  673         │ 673           │ 673           │ 673           │         │
                                  75.31 KB    │ 75.31 KB      │ 75.31 KB      │ 75.31 KB      │         │
```

### Custom benchmarks

```
Advanced Routing/mine
  time:   [1.1675 µs 1.1696 µs 1.1720 µs]

Advanced Routing/regex
  time:   [1.6611 µs 1.6660 µs 1.6714 µs]

advanced_divan       fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ matches                         │               │               │               │         │
   ├─ mine_matches   1.166 µs      │ 1.353 µs      │ 1.197 µs      │ 1.212 µs      │ 100     │ 400
   │                 alloc:        │               │               │               │         │
   │                   1           │ 1             │ 1             │ 1             │         │
   │                   256 B       │ 256 B         │ 256 B         │ 256 B         │         │
   │                 dealloc:      │               │               │               │         │
   │                   1           │ 1             │ 1             │ 1             │         │
   │                   512 B       │ 512 B         │ 512 B         │ 512 B         │         │
   │                 grow:         │               │               │               │         │
   │                   1           │ 1             │ 1             │ 1             │         │
   │                   256 B       │ 256 B         │ 256 B         │ 256 B         │         │
   ╰─ regex_matches  1.624 µs      │ 188.6 µs      │ 5.083 µs      │ 5.873 µs      │ 100     │ 100
                     alloc:        │               │               │               │         │
                       14          │ 281           │ 14            │ 16.67         │         │
                       406 B       │ 114.1 KB      │ 406 B         │ 1.543 KB      │         │
                     dealloc:      │               │               │               │         │
                       14          │ 23            │ 14            │ 14.09         │         │
                       406 B       │ 13.18 KB      │ 406 B         │ 533.7 B       │         │
                     grow:         │               │               │               │         │
                       0           │ 20            │ 0             │ 0.2           │         │
                       0 B         │ 70.23 KB      │ 0 B           │ 702.3 B       │         │
```
g
