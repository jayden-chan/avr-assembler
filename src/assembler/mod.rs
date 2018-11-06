//!
//! This module is responsible for assembling the pre-processed
//! code
//!
use std::collections::HashMap;

mod directives;
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

impl Interm {
    pub fn reset_counters(&mut self) {
        self.locctr = 0;
        self.linectr = 0;
    }
}

///
/// This function completes the first pass of the algorithm.
/// General description is available in the PDF.
///
/// Note: This function will mutate the `interm` parameter.
///
pub fn first_pass(file: &String, interm: &mut Interm) -> Result<(), String> {
    interm.reset_counters();

    for line in file.lines() {
        let mut tokens: Vec<_> = line.split_whitespace().collect();

        interm.linectr += 1;
        println!("{} ({}): {}", interm.linectr, interm.locctr, line);

        // Skip blank lines
        if tokens.len() == 0 {
            continue;
        }

        // Handle comments and assembler directives
        match &tokens[0][..1] {
            ";" | "#" => continue,
            "." => match directives::handle(line.to_string(), interm) {
                Ok(_) => continue,
                Err(e) => {
                    return Err(e);
                }
            },
            _ => {}
        }

        if tokens[0].ends_with(":") {
            let symbol = &tokens[0][..tokens[0].len() - 1];

            if interm.symtab.contains_key(symbol) {
                return Err(format!(
                    "Error: redefinition of symbol \"{}\"\nLine {}:\n\n{}",
                    symbol, interm.linectr, line
                ));
            } else {
                interm.symtab.insert(symbol.to_string(), interm.locctr);
            }
        } else {
            interm.locctr += op::length(tokens[0]);
        }
    }

    Ok(())
}

///
/// This function completes the second pass of the algorithm.
/// General description is available in the PDF.
///
/// Note: This function will mutate the `interm` parameter.
///
pub fn second_pass(file: &String, interm: &mut Interm) -> Result<(), String> {
    interm.reset_counters();

    'outer: for line in file.lines() {
        let mut tokens: Vec<_> = line.split_whitespace().collect();

        interm.linectr += 1;
        println!("{}: {}", interm.linectr, line);

        // Skip blank lines
        if tokens.len() == 0 {
            continue;
        }

        // Skip commented lines and assembler directives
        match &tokens[0][..1] {
            ";" | "." | "#" => continue,
            _ => {},
        }

        for token in tokens {
            if &token[..1] == ";" {
                continue 'outer;
            }

            println!("parsing. token: {}", token);
        }
    }

    Ok(())
}
