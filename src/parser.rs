use crate::architecture;

/// Parses a 16 character immediate into a number representation
/// hex_str: The 16 char immediate
pub fn parse_hexadecimal(hex_str: &str) -> u16 {
    let hex_str = hex_str.replace("0x", "");
    let mut result: u16 = 0;
    for (i, c) in hex_str.chars().enumerate() {
        let representation: u16 = c.to_digit(16).unwrap() as u16;
        let power = (hex_str.len() - i - 1) as u32;
        result += 16u16.pow(power) * representation;
    }
    result
}

pub fn parse_function(instruction: String) -> Option<architecture::Instruction> {
    let i_types = vec!["addi", "addiu", "slti", "sltiu", "andi", "ori", "xori", "lui"];
    for instr_type in &i_types {
        if instruction.starts_with(instr_type) {
            let instr = architecture::Instruction { instruction: instruction.clone(), itype: architecture::InstructionType::IType };
            return Some(instr);
        }
    }
    return None
}

pub fn parse_register(register: &str) -> u8 {
    let result = match register {
        "$t0" => 8,
        "$t1" => 9,
        "$t2" => 10,
        "$t3" => 11,
        "$t4" => 12,
        "$t5" => 13,
        "$t6" => 14,
        "$t7" => 15,
        "$t8" => 16,
        "$t9" => 17,
        _ => {
            panic!("Invalid register type!");
        }
    };
    result
}