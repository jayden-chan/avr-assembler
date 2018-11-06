//!
//! This module is responsible for handling the assembler directives
//! such as .cseg, .dseg, .INCLUDE, etc..
//!

use assembler::Interm;

use std::u32;

///
/// Handle parses a line with an assembler directive and modifies
/// the interm accordingly.
///
pub fn handle(line: String, interm: &mut Interm) -> Result<(), String> {
    let tokens: Vec<_> = line.split_whitespace().collect();

    if tokens.len() == 0 {
        return Ok(());
    }

    match tokens[0] {
        ".org" => {
            if tokens.len() <= 1 {
                return Err(format!(
                        "Error parsing .org directive: {}\nLine {}:\n\n{}",
                        "no argument provided", interm.linectr, line
                ));
            }

            match num_from_str(tokens[1]) {
                Ok(n) => interm.locctr = n,
                Err(e) =>  {
                    return Err(format!(
                            "Error parsing .org directive: {}\nLine {}:\n\n{}",
                            e, interm.linectr, line
                    ));
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn num_from_str(string: &str) -> Result<u32, String> {
    match &string[..2] {
        "0x" => {
            match u32::from_str_radix(&string[2..], 16) {
                Ok(n) => return Ok(n),
                Err(e) => return Err(e.to_string())
            }
        }
        "0b" => {
            match u32::from_str_radix(&string[2..], 2) {
                Ok(n) => return Ok(n),
                Err(e) => return Err(e.to_string())
            }
        }
        _ => {
            match string.parse::<u32>() {
                Ok(n) => return Ok(n),
                Err(e) => return Err(e.to_string())
            }
        }
    }
}
