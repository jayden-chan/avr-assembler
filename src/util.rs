use std::u32;

///
/// Takes in a number in either base 2, 10 or 16 and returns
/// the binary value.
///
pub fn num_from_str(string: String) -> Result<u32, String> {
    match &string[..2] {
        "0x" => match u32::from_str_radix(&string[2..], 16) {
            Ok(n) => return Ok(n),
            Err(e) => return Err(e.to_string()),
        },
        "0b" => match u32::from_str_radix(&string[2..], 2) {
            Ok(n) => return Ok(n),
            Err(e) => return Err(e.to_string()),
        },
        _ => match string.parse::<u32>() {
            Ok(n) => return Ok(n),
            Err(e) => return Err(e.to_string()),
        },
    }
}

///
/// Splits a string using whitespace or commas as
/// the split characters. Also removes empty strings
/// from the resulting split vector
///
pub fn split_string<'a>(line: &'a String) -> Vec<&'a str> {
    line.split(|c: char| c == ',' || c.is_whitespace()).filter(|i| i != &"").collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_from_str() {
        assert_eq!(num_from_str(String::from("10")), Ok(10));
        assert_eq!(num_from_str(String::from("0xFF")), Ok(0xFF));
        assert_eq!(num_from_str(String::from("0b1000")), Ok(0b1000));
        assert_eq!(num_from_str(String::from("wuadbu")), Err(String::from("invalid digit found in string")));
        assert_eq!(num_from_str(String::from("0xTT")), Err(String::from("invalid digit found in string")));
        assert_eq!(num_from_str(String::from("0bJJJ")), Err(String::from("invalid digit found in string")));
    }

    #[test]
    fn test_split_string() {
        assert_eq!(split_string(
                &String::from("test: ldi r16, 9 ;this is a comment")),
                vec!["test:", "ldi", "r16", "9", ";this", "is", "a", "comment"]);

        assert_eq!(split_string(
                &String::from("this is a nother, r, test ; testing here")),
                vec!["this", "is", "a", "nother", "r", "test", ";", "testing", "here"]);

        assert_eq!(split_string(
                &String::from("testing,just,commas,here")),
                vec!["testing", "just", "commas", "here"]);

        assert_eq!(split_string(
                &String::from("testing just spaces here")),
                vec!["testing", "just", "spaces", "here"]);
    }
}
