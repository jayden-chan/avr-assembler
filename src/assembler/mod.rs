///
/// This module is responsible for assembling the pre-processed
/// code
///
use std::collections::HashMap;

mod optab;

#[derive(Debug)]
pub struct Line {
    num: i32,
    addr: i32,
    ins: optab::Instruction,
    opcode: i32,
}

#[derive(Debug)]
pub struct Interm {
    pub lines: Vec<Line>,
    pub optab: Vec<String>,
    pub locctr: i32,
    pub linectr: i32,
    pub symtab: HashMap<String, i32>,
}

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

        if tokens.len() == 0 {
            continue;
        }

        let first = &tokens[0][..1];
        match first {
            ";" | "." => continue,
            _ => {},
        }

        if tokens[0].ends_with(":") {
            let symbol = &tokens[0][..tokens[0].len()-1];

            if ret.symtab.contains_key(symbol) {
                return Err(format!("Error line {}: Redefinition of symbol \"{}\"", ret.linectr, symbol));
            } else {
                ret.symtab.insert(symbol.to_string(), ret.locctr);
            }
        }

        ret.locctr += 1;
    }

    Ok(ret)
}
