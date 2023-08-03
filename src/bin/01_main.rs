/*  name = 01.main
    build = cargo build --bin 01_main
    run = cargo run --bin 01_main
    test = cargo test --bin 01_main
    bench = cargo bench --bin 01_main
*/

#![feature(test)]
fn main() {
    println!("Hello, TTF_RUST!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}