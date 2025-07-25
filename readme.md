### Fast String Contains

## Benchmarks

## Criterion Benchmarks

```text
string contains         time:   [7.7557 ns 7.7629 ns 7.7716 ns]
                        change: [-0.9531% -0.0875% +0.8345%] (p = 0.86 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
```

```text
fast contains           time:   [5.0054 ns 5.0092 ns 5.0133 ns]
                        change: [-1.0042% +0.0536% +1.0664%] (p = 0.93 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe
```

## Instant::now() Benchmark

<details>
<summary>Benchmark Code</summary>

```rust
fn main() {
    let haystack = "The quick brown fox jumps over the lazy dog";
    let needle = "fox";
    let iterations = 1000000;
    use std::time::Instant;
    let start = Instant::now();
    for _ in 0..iterations {
        if haystack.fast_contains(needle) {};
    }
    let duration = start.elapsed();
    println!(
        "Time taken for {} iterations of fast_contains: {:?}",
        iterations, duration
    );

    let start_std = Instant::now();
    for _ in 0..iterations {
        if haystack.contains(needle) {};
    }
    let duration_std = start_std.elapsed();
    println!(
        "Time taken for {} iterations of contains: {:?}",
        iterations, duration_std
    );
}
```
</details>

```text
Time taken for 1000000 iterations of fast_contains: 75.262571ms
Time taken for 1000000 iterations of contains: 623.253574ms
```

## Using the Library
Its as simple as adding the following to your `Cargo.toml`:

```toml
[dependencies]
fastcontains = "1.0.0"
```

and then importing it in your code:

```rust
use fastcontains::ConFast;
```
