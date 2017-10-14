pub fn encode(mut input: u32) -> String {

    let mapping = [
        ("V", 5),
        ("I", 1)
    ];

    let mut result = String::new();

    while input > 0 {
        for &(roman, decimal) in &mapping {
            if input >= decimal {
                result = result + roman;
                input = input - decimal;
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
    fn encode_6() {
        assert_eq!("VI", encode(6));
    }
}
