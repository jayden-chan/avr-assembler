///
/// This module is responsible for assembling the pre-processed
/// code
///
use std::collections::HashMap;

mod op;

#[derive(Debug)]
pub struct Line {
    pub num: u32,
    pub addr: u32,
    pub ins: op::Instruction,
    pub opcode: i32,
}

#[derive(Debug)]
pub struct Interm {
    pub lines: Vec<Line>,
    pub optab: Vec<String>,
    pub locctr: u32,
    pub linectr: u32,
    pub symtab: HashMap<String, u32>,
}

///
/// This function completes the first pass of the algorithm.
/// General description is available in the PDF.
///
/// Note: This function will mutate the `interm` parameter.
///
pub fn first_pass(file: &String, interm: &mut Interm) -> Result<(), String> {
    for line in file.lines() {
        let mut tokens: Vec<_> = line.split_whitespace().collect();

        interm.linectr += 1;
        println!("{}: {}", interm.linectr, line);

        // Skip blank lines
        if tokens.len() == 0 {
            continue;
        }

        // Skip commented lines and assembler directives (for now)
        match &tokens[0][..1] {
            ";" | "." | "#" => continue,
            _ => {},
        }

        for token in tokens {
            if token.ends_with(":") {
                let symbol = &token[..token.len()-1];

                if interm.symtab.contains_key(symbol) {
                    return Err(
                        format!(
                            "Error: redefinition of symbol \"{}\"\nLine {}:\n\n{}",
                            symbol,
                            interm.linectr,
                            line
                        )
                    );
                } else {
                    interm.symtab.insert(symbol.to_string(), interm.locctr);
                }
            } else {
                interm.locctr += op::length(token);
                break;
            }
        }

    }

    Ok(())
}
