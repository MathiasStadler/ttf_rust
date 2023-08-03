/*
    # run in bash
    clear;
    export NAME=00_test_println;
    echo "clean => $NAME";cargo clean --verbose --color  always;
    echo "build => $NAME";cargo build --bin $NAME;
    echo "run => $NAME";cargo run --bin $NAME;
    echo "test => $NAME";cargo test --bin $NAME -- --show-output --color always;
    echo "bench => $NAME";cargo bench --bin $NAME;
*/
#![feature(test)]
#[inline]
#[allow(dead_code)]

fn main() {
    println!("Hello, test_println!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("test_println");
        assert_eq!("test_println", "test_println");
    }
    /* for later
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
    */
}
