pub fn encode(mut input: u32) -> String {

    let mapping = [
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

    let mut result = String::new();

    while input > 0 {
        for &(roman, decimal) in &mapping {
            if input >= decimal {
                result = result + roman;
                input = input - decimal;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_1() {
        assert_eq!("I", encode(1));
    }

    #[test]
    fn encode_2() {
        assert_eq!("II", encode(2));
    }

    #[test]
    fn encode_4() {
        assert_eq!("IV", encode(4));
    }

    #[test]
    fn encode_6() {
        assert_eq!("VI", encode(6));
    }

    #[test]
    fn encode_1991() {
        assert_eq!("MCMXCI", encode(1991));
    }

    #[test]
    fn encode_4999() {
        assert_eq!("MMMMCMXCIX", encode(4999));
    }

    #[test]
    fn encode_444() {
        assert_eq!("CDXLIV", encode(444));
    }
}
