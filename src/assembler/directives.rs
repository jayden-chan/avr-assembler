//!
//! This module is responsible for handling the assembler directives
//! such as .cseg, .dseg, .INCLUDE, etc..
//!

use assembler::Interm;
use util;

///
/// Handle parses a line with an assembler directive and modifies
/// the interm accordingly.
///
pub fn handle(line: String, interm: &mut Interm) -> Result<(), String> {
    let tokens = util::split_string(&line);

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

            match util::num_from_str(tokens[1].to_string()) {
                Ok(n) => interm.locctr = n,
                Err(e) => {
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
