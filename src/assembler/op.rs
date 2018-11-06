//!
//! The op module contains functions related to assembly
//! operation codes
//!

///
/// Instruction mnemonics borrowed from `avra`
///
#[derive(Debug)]
pub struct Instruction {
    mnemonic: String,
    opcode: u32,
}

///
/// Length returns the length of the opcode for the
/// provided instruction code. This function does not
/// produce any kind of error if an invalid instruction is
/// provided.
///
pub fn length(code: &str) -> u32 {
    match code {
        "CALL" | "JMP" | "LDS" => 32,
        _ => 16,
    }
}
