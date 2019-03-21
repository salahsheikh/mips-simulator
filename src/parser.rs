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
    let result: u8 = match register {
        "$zero" => 0,
        "$at" => 1,
        "$v0" => 2,
        "$v1" => 3,
        "$a0" => 4,
        "$a1" => 5,
        "$a2" => 6,
        "$a3" => 7,
        "$t0" => 8,
        "$t1" => 9,
        "$t2" => 10,
        "$t3" => 11,
        "$t4" => 12,
        "$t5" => 13,
        "$t6" => 14,
        "$t7" => 15,
        "$s0" => 16,
        "$s1" => 17,
        "$s2" => 18,
        "$s3" => 19,
        "$s4" => 20,
        "$s5" => 21,
        "$s6" => 22,
        "$s7" => 23,
        "$t8" => 24,
        "$t9" => 25,
        "$k0" => 26,
        "$k1" => 27,
        "$gp" => 28,
        "$sp" => 29,
        "$fp" => 30,
        "$ra" => 31,
        _ => {
            panic!("Invalid register type!");
        }
    };
    result
}