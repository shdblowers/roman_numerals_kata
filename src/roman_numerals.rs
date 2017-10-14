pub fn encode(mut input: u32) -> String {

    let mapping = [
        ("V", 5),
        ("I", 1)
    ];

    let mut result = String::new();

    while input > 0 {

        for map in &mapping {
            if input >= map.1 {
                result = result + map.0;
                input = input - map.1;
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
