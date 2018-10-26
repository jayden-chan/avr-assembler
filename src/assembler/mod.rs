///
/// This module is responsible for assembling the pre-processed
/// code
///
use std::collections::HashMap;

mod op;

#[derive(Debug)]
struct Line {
    num: u32,
    addr: u32,
    ins: op::Instruction,
    opcode: i32,
}

#[derive(Debug)]
pub struct Interm {
    lines: Vec<Line>,
    optab: Vec<String>,
    locctr: u32,
    linectr: u32,
    symtab: HashMap<String, u32>,
}

///
/// This function completes the first pass of the algorithm.
/// General description is available in the PDF
///
pub fn first_pass(file: String) -> Result<Interm, String> {
    let mut ret = Interm {
        lines: Vec::new(),
        optab: Vec::new(),
        locctr: 0,
        linectr: 0,
        symtab: HashMap::new(),
    };

    for line in file.lines() {
        let mut tokens: Vec<_> = line.split_whitespace().collect();

        ret.linectr += 1;
        println!("{}: {}", ret.linectr, line);

        // Skip blank lines
        if tokens.len() == 0 {
            continue;
        }

        // Skip commented lines and assembler directives (for now)
        let first = &tokens[0][..1];
        match first {
            ";" | "." => continue,
            _ => {},
        }

        if tokens[0].ends_with(":") {
            let symbol = &tokens[0][..tokens[0].len()-1];

            if ret.symtab.contains_key(symbol) {
                return Err(
                    format!(
                        "Error: redefinition of symbol \"{}\"\nLine {}:\n\n{}",
                        symbol,
                        ret.linectr,
                        line
                    )
                );
            } else {
                ret.symtab.insert(symbol.to_string(), ret.locctr);
            }
        }

        ret.locctr += op::length(&tokens[0]);
    }

    Ok(ret)
}
