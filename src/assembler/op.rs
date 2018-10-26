#[derive(Debug)]
pub struct Instruction {
    code: String,
    operands: Vec<String>,
}

///
/// Length returns the length of the opcode for the
/// provided instruction code
///
pub fn length(code: &str) -> u32 {

    match code {
        "CALL" | "JMP" | "LDS" => 32,
        _ => 16,
    }

    // 32 bit opcodes:
    // CALL
    // JMP
    // LDS

    // Weird opcodes
    // LD (LDD)
    // ST (STD)
}
