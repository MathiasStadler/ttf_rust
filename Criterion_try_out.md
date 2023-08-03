# Criterion try out

[FROM HERE](https://bheisler.github.io/criterion.rs/book/getting_started.html)

## Test in a new project

Step 1 - add criterion to the project

```bash
cargo add criterion
cargo build
```

Step 2 - add example from tutorial

Step 2.1 - create test code inside src/lib.rs

```bash
cat <<EOF >./src/lib.rs
#[inline]
#[allow(dead_code)]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}
EOF
```

Step 3.1 - add new folder benches

```bash
# inside project folder
mkdir benches
```

Step 3.2 -  add new rust file

```bash
cat <<EOF >./benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mycrate::fibonacci;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
EOF
```

Step 4 - Run Benchmark

```bash
cargo bench
```

- output

```bash
cargo bench
   Compiling ttf_rust v0.1.0 (/home/trapapa/ttf_rust)
    Finished bench [optimized] target(s) in 4.96s
     Running unittests src/lib.rs (target/release/deps/ttf_rust-88650684e06cd0e1)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/00_test_println.rs (target/release/deps/00_test_println-20923102990247e8)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/01_main.rs (target/release/deps/01_main-4abf057e28b9d5b6)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-9f153ff338b2de31)
Gnuplot not found, using plotters backend
fib 20                  time:   [26.121 µs 26.760 µs 27.557 µs]
Found 16 outliers among 100 measurements (16.00%)
  4 (4.00%) high mild
  12 (12.00%) high severe
```