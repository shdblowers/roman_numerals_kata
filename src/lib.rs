pub mod encoder;

pub fn parse(args: &[String]) -> Result<u32, &'static str> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_converts_command_line_args_to_single_number() {
        assert_eq!(Ok(29), parse(&["cli_app_name".to_string(), "29".to_string()]));
    }

    #[test]
    fn parse_errors_if_provided_with_too_few_arguments() {
        assert_eq!(Err("Incorrect number of arguments."), parse(&["cli_app_name".to_string()]));
    }

    #[test]
    fn parse_errors_if_provided_with_too_many_arguments() {
        assert_eq!(
            Err("Incorrect number of arguments."),
            parse(&["cli_app_name".to_string(), "34".to_string(), "999".to_string()])
        );
    }

    #[test]
    fn parse_errors_when_its_not_given_a_number() {
        assert_eq!(
            Err("Argument not a number."),
            parse(&["cli_app_name".to_string(), "nine".to_string()])
        );
    }

    #[test]
    fn parse_errors_when_number_too_large() {
        assert_eq!(
            Err("Number must be between 1 and 3999."),
            parse(&["cli_app_name".to_string(), "5001".to_string()])
        );
    }

    #[test]
    fn parse_errors_when_number_too_small() {
        assert_eq!(
            Err("Number must be between 1 and 3999."),
            parse(&["cli_app_name".to_string(), "0".to_string()])
        );
    }
}
