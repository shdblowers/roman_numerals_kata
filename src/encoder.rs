pub fn run(mut input: u32) -> String {

    let mut result = String::new();

    while input > 0 {
        for &(roman, decimal) in &MAPPING {
            if input >= decimal {
                result = result + roman;
                input = input - decimal;
                break;
            }
        }
    }

    result
}

const MAPPING: [(&str, u32); 13] = [
    ("M",  1000),
    ("CM", 900),
    ("D",  500),
    ("CD", 400),
    ("C",  100),
    ("XC", 90),
    ("L",  50),
    ("XL", 40),
    ("X",  10),
    ("IX", 9),
    ("V",  5),
    ("IV", 4),
    ("I",  1),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_singles() {
        assert_eq!("I", run(1));
        assert_eq!("X", run(10));
        assert_eq!("C", run(100));
        assert_eq!("MCXI", run(1111));
    }

    #[test]
    fn encode_fours() {
        assert_eq!("IV", run(4));
        assert_eq!("XL", run(40));
        assert_eq!("CD", run(400));
        assert_eq!("MMMM", run(4000));
    }

    #[test]
    fn encode_fives() {
        assert_eq!("V", run(5));
        assert_eq!("L", run(50));
        assert_eq!("D", run(500));
    }

    #[test]
    fn encode_nines() {
        assert_eq!("IX", run(9));
        assert_eq!("XC", run(90));
        assert_eq!("CM", run(900));
    }

    #[test]
    fn encode_multiples() {
        assert_eq!("CDXLIV", run(444));
        assert_eq!("MCMXCI", run(1991));
        assert_eq!("MMMMDXCIV", run(4594));
        assert_eq!("MMMMCMXCIX", run(4999));
    }
}
