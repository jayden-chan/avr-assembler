//!
//! The op module contains functions related to assembly
//! operation codes
//!
use std::collections::HashMap;
use assembler::Interm;

use util;

#[derive(Debug)]
pub struct Instruction {
    index: u16,
    opcode: u32,
}

#[derive(Debug)]
enum ObjectCode {
    Short(u16),
    Long(u32),
}

///
/// Length returns the length of the opcode for the
/// provided instruction code. This function does not
/// produce any kind of error if an invalid instruction is
/// provided.
///
pub fn length(code: &str) -> u32 {
    match code {
        "call" | "jmp" | "lds" => 32,
        _ => 16,
    }
}

///
/// Parses one instruction and returns the operands.
/// This function will translate register symbols as
/// well as symbols from the SYMTAB. This function assumes
/// that the input string will be in the format [instruction] [operands]...
///
pub fn get_operands(line: String, interm: &Interm) -> Result<Vec<u32>, String> {
    let tokens = util::split_string(&line);
    let mut ret = Vec::new();

    for token in &tokens[1..] {
        if interm.instructions.contains_key(token) {
            continue;
        }

        let mut token = token.to_string();

        if token.ends_with(",") {
            token.pop();
        }


        match token.chars().next().unwrap() {
            'r' => {
                let result = reg_to_num(token);

                match result {
                    Ok(n) => ret.push(n),
                    Err(e) => return Err(e)
                }
            }

            '0'...'9' => {
                match util::num_from_str(token) {
                    Ok(n) => ret.push(n),
                    Err(e) => return Err(e)
                }
            },

            ';' => return Ok(ret),

            _ => {
                match interm.symtab.get(token.as_str()) {
                    Some(&n) => ret.push(n),
                    None => return Err(format!("Undefined symbol {}", token))
                }
            }
        }
    }

    Ok(ret)
}

///
/// Takes a string in the form r[d][d] and returns the integer
/// representation
///
pub fn reg_to_num(reg: String) -> Result<u32, String> {
    match reg[1..].parse::<u32>() {
        Ok(n) => {
            match n {
                0...31 => {
                    return Ok(n);
                }
                _ => {
                    return Err(format!("Register number out of range ({})", n));
                }
            }
        }
        Err(e) => return Err(format!("Failed to parse register number: {}", e))
    }
}

///
/// Assembles one instruction and returns the
/// binary representation
///
fn parse(ins: Instruction) -> ObjectCode {
    match ins.index {
        0...27 => {
            println!("<= break");
        }

        28 | 29 => {
            println!("<= elpm");
        }

        index => {
            // check for operand 1 and 2 here

            if index >= 69 {
                println!(">= brbs");
            }

            match index {
                0...31 => {
                    println!("<= bclr");
                }

                32...46 => {
                    println!("<= rol");
                }

                47...66 => {
                    println!("<= rcall");
                }

                67 | 68  => {
                    println!("<= call");
                }

                69 | 70 => {
                    println!("<= brbc");
                }

                71...82 => {
                    println!("<= mul");
                }

                83 => {
                    println!("<= movw");
                }

                84 => {
                    println!("<= muls");
                }

                85...88 => {
                    println!("<= fmulsu");
                }

                89 | 90 => {
                    println!("<= sbiw");
                }

                91...98 => {
                    println!("<= cbr");
                }

                99...102 => {
                    println!("<= bld");
                }

                103 => {
                    println!("in");
                }

                104 => {
                    println!("out");
                }

                105...108 => {
                    println!("<= cbi");
                }

                109 => {
                    println!("lds");
                }

                110 => {
                    println!("sts");
                }

                111 => {
                    println!("ld");
                }

                112 => {
                    println!("st");
                }

                113 => {
                    println!("ldd");
                }

                114 => {
                    println!("std");
                }

                i => {
                    println!("else {}", i);
                }

            }

        }

    }

    return ObjectCode::Short(0x200);
}

///
/// This function initializes the list of available instructions
/// since we cannot (yet) have static HashMaps in Rust. Instruction
/// list borrowed from `avra`
///
pub fn init_op_map(interm: &mut Interm) {
    interm.instructions = HashMap::new();

    interm.instructions.insert("nop",   Instruction{opcode: 0x0000, index: 0});
    interm.instructions.insert("sec",   Instruction{opcode: 0x9408, index: 1});
    interm.instructions.insert("clc",   Instruction{opcode: 0x9488, index: 2});
    interm.instructions.insert("sen",   Instruction{opcode: 0x9428, index: 3});
    interm.instructions.insert("cln",   Instruction{opcode: 0x94a8, index: 4});
    interm.instructions.insert("sez",   Instruction{opcode: 0x9418, index: 5});
    interm.instructions.insert("clz",   Instruction{opcode: 0x9498, index: 6});
    interm.instructions.insert("sei",   Instruction{opcode: 0x9478, index: 7});
    interm.instructions.insert("cli",   Instruction{opcode: 0x94f8, index: 8});
    interm.instructions.insert("ses",   Instruction{opcode: 0x9448, index: 9});
    interm.instructions.insert("cls",   Instruction{opcode: 0x94c8, index: 10});
    interm.instructions.insert("sev",   Instruction{opcode: 0x9438, index: 11});
    interm.instructions.insert("clv",   Instruction{opcode: 0x94b8, index: 12});
    interm.instructions.insert("set",   Instruction{opcode: 0x9468, index: 13});
    interm.instructions.insert("clt",   Instruction{opcode: 0x94e8, index: 14});
    interm.instructions.insert("seh",   Instruction{opcode: 0x9458, index: 15});
    interm.instructions.insert("clh",   Instruction{opcode: 0x94d8, index: 16});
    interm.instructions.insert("sleep", Instruction{opcode: 0x9588, index: 17});
    interm.instructions.insert("wdr",   Instruction{opcode: 0x95a8, index: 18});
    interm.instructions.insert("ijmp",  Instruction{opcode: 0x9409, index: 19});
    interm.instructions.insert("eijmp", Instruction{opcode: 0x9419, index: 20});
    interm.instructions.insert("icall", Instruction{opcode: 0x9509, index: 21});
    interm.instructions.insert("eicall",Instruction{opcode: 0x9519, index: 22});
    interm.instructions.insert("ret",   Instruction{opcode: 0x9508, index: 23});
    interm.instructions.insert("reti",  Instruction{opcode: 0x9518, index: 24});
    interm.instructions.insert("spm",   Instruction{opcode: 0x95e8, index: 25});
    interm.instructions.insert("espm",  Instruction{opcode: 0x95f8, index: 26});
    interm.instructions.insert("break", Instruction{opcode: 0x9598, index: 27});
    interm.instructions.insert("lpm",   Instruction{opcode: 0x95c8, index: 28});
    interm.instructions.insert("elpm",  Instruction{opcode: 0x95d8, index: 29});
    interm.instructions.insert("bset",  Instruction{opcode: 0x9408, index: 30});
    interm.instructions.insert("bclr",  Instruction{opcode: 0x9488, index: 31});
    interm.instructions.insert("ser",   Instruction{opcode: 0xef0f, index: 32});
    interm.instructions.insert("com",   Instruction{opcode: 0x9400, index: 33});
    interm.instructions.insert("neg",   Instruction{opcode: 0x9401, index: 34});
    interm.instructions.insert("inc",   Instruction{opcode: 0x9403, index: 35});
    interm.instructions.insert("dec",   Instruction{opcode: 0x940a, index: 36});
    interm.instructions.insert("lsr",   Instruction{opcode: 0x9406, index: 37});
    interm.instructions.insert("ror",   Instruction{opcode: 0x9407, index: 38});
    interm.instructions.insert("asr",   Instruction{opcode: 0x9405, index: 39});
    interm.instructions.insert("swap",  Instruction{opcode: 0x9402, index: 40});
    interm.instructions.insert("push",  Instruction{opcode: 0x920f, index: 41});
    interm.instructions.insert("pop",   Instruction{opcode: 0x900f, index: 42});
    interm.instructions.insert("tst",   Instruction{opcode: 0x2000, index: 43});
    interm.instructions.insert("clr",   Instruction{opcode: 0x2400, index: 44});
    interm.instructions.insert("lsl",   Instruction{opcode: 0x0c00, index: 45});
    interm.instructions.insert("rol",   Instruction{opcode: 0x1c00, index: 46});
    interm.instructions.insert("breq",  Instruction{opcode: 0xf001, index: 47});
    interm.instructions.insert("brne",  Instruction{opcode: 0xf401, index: 48});
    interm.instructions.insert("brcs",  Instruction{opcode: 0xf000, index: 49});
    interm.instructions.insert("brcc",  Instruction{opcode: 0xf400, index: 50});
    interm.instructions.insert("brsh",  Instruction{opcode: 0xf400, index: 51});
    interm.instructions.insert("brlo",  Instruction{opcode: 0xf000, index: 52});
    interm.instructions.insert("brmi",  Instruction{opcode: 0xf002, index: 53});
    interm.instructions.insert("brpl",  Instruction{opcode: 0xf402, index: 54});
    interm.instructions.insert("brge",  Instruction{opcode: 0xf404, index: 55});
    interm.instructions.insert("brlt",  Instruction{opcode: 0xf004, index: 56});
    interm.instructions.insert("brhs",  Instruction{opcode: 0xf005, index: 57});
    interm.instructions.insert("brhc",  Instruction{opcode: 0xf405, index: 58});
    interm.instructions.insert("brts",  Instruction{opcode: 0xf006, index: 59});
    interm.instructions.insert("brtc",  Instruction{opcode: 0xf406, index: 60});
    interm.instructions.insert("brvs",  Instruction{opcode: 0xf003, index: 61});
    interm.instructions.insert("brvc",  Instruction{opcode: 0xf403, index: 62});
    interm.instructions.insert("brie",  Instruction{opcode: 0xf007, index: 63});
    interm.instructions.insert("brid",  Instruction{opcode: 0xf407, index: 64});
    interm.instructions.insert("rjmp",  Instruction{opcode: 0xc000, index: 65});
    interm.instructions.insert("rcall", Instruction{opcode: 0xd000, index: 66});
    interm.instructions.insert("jmp",   Instruction{opcode: 0x940c, index: 67});
    interm.instructions.insert("call",  Instruction{opcode: 0x940e, index: 68});
    interm.instructions.insert("brbs",  Instruction{opcode: 0xf000, index: 69});
    interm.instructions.insert("brbc",  Instruction{opcode: 0xf400, index: 70});
    interm.instructions.insert("add",   Instruction{opcode: 0x0c00, index: 71});
    interm.instructions.insert("adc",   Instruction{opcode: 0x1c00, index: 72});
    interm.instructions.insert("sub",   Instruction{opcode: 0x1800, index: 73});
    interm.instructions.insert("sbc",   Instruction{opcode: 0x0800, index: 74});
    interm.instructions.insert("and",   Instruction{opcode: 0x2000, index: 75});
    interm.instructions.insert("or",    Instruction{opcode: 0x2800, index: 76});
    interm.instructions.insert("eor",   Instruction{opcode: 0x2400, index: 77});
    interm.instructions.insert("cp",    Instruction{opcode: 0x1400, index: 78});
    interm.instructions.insert("cpc",   Instruction{opcode: 0x0400, index: 79});
    interm.instructions.insert("cpse",  Instruction{opcode: 0x1000, index: 80});
    interm.instructions.insert("mov",   Instruction{opcode: 0x2c00, index: 81});
    interm.instructions.insert("mul",   Instruction{opcode: 0x9c00, index: 82});
    interm.instructions.insert("movw",  Instruction{opcode: 0x0100, index: 83});
    interm.instructions.insert("muls",  Instruction{opcode: 0x0200, index: 84});
    interm.instructions.insert("mulsu", Instruction{opcode: 0x0300, index: 85});
    interm.instructions.insert("fmul",  Instruction{opcode: 0x0308, index: 86});
    interm.instructions.insert("fmuls", Instruction{opcode: 0x0380, index: 87});
    interm.instructions.insert("fmulsu",Instruction{opcode: 0x0388, index: 88});
    interm.instructions.insert("adiw",  Instruction{opcode: 0x9600, index: 89});
    interm.instructions.insert("sbiw",  Instruction{opcode: 0x9700, index: 90});
    interm.instructions.insert("subi",  Instruction{opcode: 0x5000, index: 91});
    interm.instructions.insert("sbci",  Instruction{opcode: 0x4000, index: 92});
    interm.instructions.insert("andi",  Instruction{opcode: 0x7000, index: 93});
    interm.instructions.insert("ori",   Instruction{opcode: 0x6000, index: 94});
    interm.instructions.insert("sbr",   Instruction{opcode: 0x6000, index: 95});
    interm.instructions.insert("cpi",   Instruction{opcode: 0x3000, index: 96});
    interm.instructions.insert("ldi",   Instruction{opcode: 0xe000, index: 97});
    interm.instructions.insert("cbr",   Instruction{opcode: 0x7000, index: 98});
    interm.instructions.insert("sbrc",  Instruction{opcode: 0xfc00, index: 99});
    interm.instructions.insert("sbrs",  Instruction{opcode: 0xfe00, index: 100});
    interm.instructions.insert("bst",   Instruction{opcode: 0xfa00, index: 101});
    interm.instructions.insert("bld",   Instruction{opcode: 0xf800, index: 102});
    interm.instructions.insert("in",    Instruction{opcode: 0xb000, index: 103});
    interm.instructions.insert("out",   Instruction{opcode: 0xb800, index: 104});
    interm.instructions.insert("sbic",  Instruction{opcode: 0x9900, index: 105});
    interm.instructions.insert("sbis",  Instruction{opcode: 0x9b00, index: 106});
    interm.instructions.insert("sbi",   Instruction{opcode: 0x9a00, index: 107});
    interm.instructions.insert("cbi",   Instruction{opcode: 0x9800, index: 108});
    interm.instructions.insert("lds",   Instruction{opcode: 0x9000, index: 109});
    interm.instructions.insert("sts",   Instruction{opcode: 0x9200, index: 110});
    interm.instructions.insert("ld",    Instruction{opcode: 0,      index: 111});
    interm.instructions.insert("st",    Instruction{opcode: 0,      index: 112});
    interm.instructions.insert("ldd",   Instruction{opcode: 0,      index: 113});
    interm.instructions.insert("std",   Instruction{opcode: 0,      index: 114});
    interm.instructions.insert("count", Instruction{opcode: 0,      index: 115});
    interm.instructions.insert("lpm",   Instruction{opcode: 0x9004, index: 116});
    interm.instructions.insert("lpm",   Instruction{opcode: 0x9005, index: 117});
    interm.instructions.insert("elpm",  Instruction{opcode: 0x9006, index: 118});
    interm.instructions.insert("elpm",  Instruction{opcode: 0x9007, index: 119});
    interm.instructions.insert("ld",    Instruction{opcode: 0x900c, index: 120});
    interm.instructions.insert("ld",    Instruction{opcode: 0x900d, index: 121});
    interm.instructions.insert("ld",    Instruction{opcode: 0x900e, index: 122});
    interm.instructions.insert("ld",    Instruction{opcode: 0x8008, index: 123});
    interm.instructions.insert("ld",    Instruction{opcode: 0x9009, index: 124});
    interm.instructions.insert("ld",    Instruction{opcode: 0x900a, index: 125});
    interm.instructions.insert("ld",    Instruction{opcode: 0x8000, index: 126});
    interm.instructions.insert("ld",    Instruction{opcode: 0x9001, index: 127});
    interm.instructions.insert("ld",    Instruction{opcode: 0x9002, index: 128});
    interm.instructions.insert("st",    Instruction{opcode: 0x920c, index: 129});
    interm.instructions.insert("st",    Instruction{opcode: 0x920d, index: 130});
    interm.instructions.insert("st",    Instruction{opcode: 0x920e, index: 131});
    interm.instructions.insert("st",    Instruction{opcode: 0x8208, index: 132});
    interm.instructions.insert("st",    Instruction{opcode: 0x9209, index: 133});
    interm.instructions.insert("st",    Instruction{opcode: 0x920a, index: 134});
    interm.instructions.insert("st",    Instruction{opcode: 0x8200, index: 135});
    interm.instructions.insert("st",    Instruction{opcode: 0x9201, index: 136});
    interm.instructions.insert("st",    Instruction{opcode: 0x9202, index: 137});
    interm.instructions.insert("ldd",   Instruction{opcode: 0x8008, index: 138});
    interm.instructions.insert("ldd",   Instruction{opcode: 0x8000, index: 139});
    interm.instructions.insert("std",   Instruction{opcode: 0x8208, index: 140});
    interm.instructions.insert("std",   Instruction{opcode: 0x8200, index: 141});
    interm.instructions.insert("end",   Instruction{opcode: 0,      index: 142});
}

#[cfg(test)]
mod test {
    use super::*;

    fn init_fake_interm() -> Interm {
        let mut interm = Interm {
            lines: Vec::new(),
            optab: Vec::new(),
            instructions: HashMap::new(),
            locctr: 0,
            linectr: 0,
            symtab: HashMap::new(),
        };

        init_op_map(&mut interm);
        interm.symtab.insert(String::from("defined_label"), 200);
        interm.symtab.insert(String::from("PORTL"), 0xDEAD);
        interm.symtab.insert(String::from("PORTB"), 0xBEEF);
        interm
    }

    #[test]
    fn test_reg_to_num() {
        assert_eq!(reg_to_num("r32".to_string()), Err("Register number out of range (32)".to_string()));
        assert_eq!(reg_to_num("daowuno".to_string()),
                   Err("Failed to parse register number: invalid digit found in string".to_string()));

        for i in 0..32 {
            let mut reg = String::from("r");
            reg.push_str(i.to_string().as_str());
            assert_eq!(reg_to_num(reg), Ok(i));
        }
    }

    #[test]
    fn test_get_operands() {
        let mut interm = init_fake_interm();

        assert_eq!(get_operands(String::from("ldi r16, 0xff"), &interm), Ok(vec![16, 0xff]));
        assert_eq!(get_operands(String::from("test:       jmp 0x23"), &interm), Ok(vec![0x23]));
        assert_eq!(get_operands(String::from("loop: nop"), &interm), Ok(vec![]));
        assert_eq!(get_operands(String::from("brne done"), &interm), Err(String::from("Undefined symbol done")));
        assert_eq!(get_operands(String::from("nop"), &interm), Ok(vec![]));
        assert_eq!(get_operands(String::from("label:"), &interm), Ok(vec![]));
        assert_eq!(get_operands(String::from("label: ldi r16, 0x22"), &interm), Ok(vec![16, 0x22]));
        assert_eq!(get_operands(String::from("label: ldi r29, 0b10001010"), &interm), Ok(vec![29, 0b10001010]));
        assert_eq!(get_operands(String::from("jmp defined_label"), &interm), Ok(vec![200]));
        assert_eq!(get_operands(String::from("jmp undefined"), &interm), Err(String::from("Undefined symbol undefined")));
        assert_eq!(get_operands(String::from("label: lds r16, PORTB"), &interm), Ok(vec![16, 0xBEEF]));
        assert_eq!(get_operands(String::from("label: out PORTL, r16"), &interm), Ok(vec![0xDEAD, 16]));
    }

    #[test]
    fn test_length() {
        assert_eq!(length("ldi"), 16);
        assert_eq!(length("out"), 16);
        assert_eq!(length("tst"), 16);
        assert_eq!(length("in"), 16);
        assert_eq!(length("breq"), 16);
        assert_eq!(length("call"), 32);
        assert_eq!(length("jmp"), 32);
        assert_eq!(length("lds"), 32);
    }
}
