/*  name = 01.main
    build = cargo build --bin 01_main
    run = cargo run --bin 01_main
    test = cargo test --bin 01_main
    bench = cargo bench --bin 01_main
*/

// FROM HERE
// https://jeffkreeftmeijer.com/rust-stdin-stdout-testing/

#![feature(test)]
#[allow(unused_imports)]
use std::io;
use std::io::Write;

fn main() -> io::Result<()> {
    let buffer = "Hallo\n".to_string();

    // io::stdin().read_to_string(&mut buffer)?;
    //org
    //io::stdout().write_all(buffer.to_uppercase().as_bytes())?;
    io::stdout().write_all(buffer.as_bytes())?;

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}