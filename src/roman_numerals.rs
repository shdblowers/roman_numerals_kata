pub fn encode(mut input: u32) -> String {
    let mut result = String::new();

    while input > 0 {
        if input > 5 {
            result = result + "V";
            input = input - 5;
        } else {
            result = result + "I";
            input = input - 1;
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
