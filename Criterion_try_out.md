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

Step 2.1 - add new folder benches

```bash
# inside project folder
mkdir benches
```

Step 2.2 -  add new rust file

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

Step3 - Run Benchmark

```bash
cargo bench
```
