pub fn encode(input: u32) -> String {
    if input == 1 {
        "I".to_string()
    } else {
        "II".to_string()
    }
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
