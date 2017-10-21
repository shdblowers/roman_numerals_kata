extern crate roman_numerals_kata;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let decimal = roman_numerals_kata::parse(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{}", roman_numerals_kata::encoder::run(decimal));
}
