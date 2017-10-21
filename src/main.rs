extern crate roman_numerals_kata;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let decimal = parse(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{}", roman_numerals_kata::encode(decimal));
}

fn parse(args: &[String]) -> Result<u32, &'static str> {
    if args.len() != 2 {
        return Err("Incorrect number of arguments.");
    }

    let decimal: u32 = match args[1].clone().trim().parse() {
        Ok(num) => num,
        Err(_) => {return Err("Argument not a number.");}
    };

    if decimal < 1 || decimal > 3999 {
        return Err("Number must be between 1 and 3999.");
    }

    Ok(decimal)
}
