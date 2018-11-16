use std::u32;

///
/// Takes in a number in either base 2, 10 or 16 and returns
/// the binary value.
///
pub fn num_from_str(string: &str) -> Result<u32, String> {
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
