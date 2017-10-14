mod roman_numerals;

use std::io;

fn main() {

    loop {
        println!("Please enter the number you wish to convert to roman numerals (1 - 3999):");

        let mut user_input = String::new();

        io::stdin().read_line(&mut user_input).expect("Error");

        let user_input: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, try again!");
                continue;
            }
        };

        println!("{}", roman_numerals::encode(user_input));
        break;
    }

}
