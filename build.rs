use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {

    let out_directory = env::var("OUT_DIR").unwrap();
    let destination_path = Path::new(&out_directory).join("implementations.rs");

    let mut file = File::create(&destination_path).unwrap();

    for u in 0..33 {
        writeln!(file, "impl<T> ::ArrayLength<T> for ::typenum::consts::U{} {{", u).unwrap();

        writeln!(file, "    type Array = [T; {}];", u).unwrap();

        writeln!(file, "}}").unwrap();
    }
}