pub fn encode(mut input: u32) -> String {
    let mut result = String::new();

    while input > 0 {
        result = result + "I";
        input = input - 1;
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
}
