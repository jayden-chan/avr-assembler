//!
//! The preproc mod is responsible for handling preprocessor directives
//!

use assembler::Interm;
use util;

///
/// The parse function processes preprocessor macros like #define,
/// #undef, etc
///
pub fn parse(file: &String, interm: &mut Interm) -> Result<(), String> {
    for line in file.lines() {
        let line = line.to_string();
        let words = util::split_string(&line);

        interm.linectr += 1;

        if words.len() > 1 && words[0] == "#DEFINE" {
            let symbol = words[1];
            if interm.symtab.contains_key(symbol) {
                return Err(format!(
                    "Error: redefinition of symbol \"{}\"\nLine {}:\n\n{}",
                    symbol, interm.linectr, line
                ));
            } else if words.len() > 2 {
                match words[2].parse::<u32>() {
                    Ok(n) => {
                        interm.symtab.insert(symbol.to_string(), n);
                    }
                    Err(e) => {
                        return Err(format!(
                            "Error parsing #DEFINE macro: {}\nLine {}:\n\n{}",
                            e, interm.linectr, line
                        ));
                    }
                }
            } else {
                interm.symtab.insert(symbol.to_string(), 0);
            }
        }
    }

    Ok(())
}
