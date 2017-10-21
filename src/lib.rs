pub fn encode(mut input: u32) -> String {

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
        assert_eq!("I", encode(1));
        assert_eq!("X", encode(10));
        assert_eq!("C", encode(100));
        assert_eq!("MCXI", encode(1111));
    }

    #[test]
    fn encode_fours() {
        assert_eq!("IV", encode(4));
        assert_eq!("XL", encode(40));
        assert_eq!("CD", encode(400));
        assert_eq!("MMMM", encode(4000));
    }

    #[test]
    fn encode_fives() {
        assert_eq!("V", encode(5));
        assert_eq!("L", encode(50));
        assert_eq!("D", encode(500));
    }

    #[test]
    fn encode_nines() {
        assert_eq!("IX", encode(9));
        assert_eq!("XC", encode(90));
        assert_eq!("CM", encode(900));
    }

    #[test]
    fn encode_multiples() {
        assert_eq!("CDXLIV", encode(444));
        assert_eq!("MCMXCI", encode(1991));
        assert_eq!("MMMMDXCIV", encode(4594));
        assert_eq!("MMMMCMXCIX", encode(4999));
    }
}
