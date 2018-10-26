///
/// The parse function processes preprocessor macros like #define,
/// #undef, etc
///

use std::collections::HashMap;

use assembler::Interm;

pub fn parse(file: &String) -> Interm {
    let ret = Interm {
        lines: Vec::new(),
        optab: Vec::new(),
        locctr: 0,
        linectr: 0,
        symtab: HashMap::new(),
    };

    for line in file.lines() {
        if line.len() != 0 {
            if line[..1] == *"#" {
                println!("Found hash");

                for word in line.split_whitespace() {
                    print!("{} ", word);
                }
                println!("");
            }
        }
    }

    ret
}
