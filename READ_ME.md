# desc TTF test in RUST - Thursday, 3 August 2023

## preparation

1. rust update to last version

```bash
rustup update
```

1.1 show the installed version

```bash
rustup show

Default host: x86_64-unknown-linux-gnu
rustup home:  /home/trapapa/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)

active toolchain
----------------

nightly-x86_64-unknown-linux-gnu (default)
rustc 1.73.0-nightly (8131b9774 2023-08-02)
```

2 setup for [bench]

- [DESC 1 FROM HERE](https://doc.rust-lang.org/1.12.1/book/benchmark-tests.html)

- [DESC 2 FROM HERE](https://seenaburns.com/benchmarking-rust-with-cargo-bench/)

> for sample add a test this to src/lib.rs

```bash
cat <<EOT > ./src/lib.rs
#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
EOT
```

## run bench

```bash
cargo bench
```

## run with the nightly rust version  

rustup run nightly cargo bench

> HINT for cargo test -- -h

cargo test --bin 00_test_println -- --show-output
cargo test --bin 00_test_println -- --show-output --color always
rustup run nightly cargo test --bin 00_test_println -- --show-output --color always   -Z unstable-options --report-time
