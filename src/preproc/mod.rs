///
/// The preproc mod is responsible for handling preprocessor directives
///
use std::collections::HashMap;

use assembler::Interm;

///
/// The parse function processes preprocessor macros like #define,
/// #undef, etc
///
pub fn parse(file: &String) -> Result<Interm, String> {
    let mut ret = Interm {
        lines: Vec::new(),
        optab: Vec::new(),
        locctr: 0,
        linectr: 0,
        symtab: HashMap::new(),
    };

    for line in file.lines() {
        ret.linectr += 1;

        let words: Vec<_> = line.split_whitespace().collect();

        if words.len() > 1 && words[0] == "#DEFINE" {
            let symbol = words[1];
            if ret.symtab.contains_key(symbol) {
                return Err(
                    format!(
                        "Error: redefinition of symbol \"{}\"\nLine {}:\n\n{}",
                        symbol,
                        ret.linectr,
                        line
                    )
                );
            } else if words.len() > 2 {
                match words[2].parse::<u32>() {
                    Ok(n) => {
                        ret.symtab.insert(symbol.to_string(), n);
                    },
                    Err(e) => {
                        return Err(
                            format!(
                                "Error parsing #DEFINE macro: {}\nLine {}:\n\n{}",
                                e,
                                ret.linectr,
                                line
                            )
                        );
                    }
                }
            } else {
                ret.symtab.insert(symbol.to_string(), 0);
            }
        }
    }

    Ok(ret)
}
