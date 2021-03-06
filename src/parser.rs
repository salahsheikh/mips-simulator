use crate::architecture;

/// Parses a 16-bit immediate into a number representation
/// hex_str: The 16-bit immediate
pub fn parse_hexadecimal(hex_str: &str) -> i32 {
    let hex_str = hex_str.replace("0x", "");
    i32::from_str_radix(hex_str.as_str(), 16).unwrap()
}

pub fn parse_function(instruction: String) -> Option<architecture::Instruction> {
    let i_types = vec!["addi", "addiu", "slti", "sltiu", "andi", "ori", "xori", "lui"];
    let r_types = vec!["and", "or", "nor"];
    let special_types = vec!["nop", "sll", "srl", "lw", "sw"];
    let j_types = vec!["j", "jr", "jal"];
    let opword: &str = instruction.split_whitespace().next().unwrap_or("");
    for instr_type in &r_types {
        if opword.eq(*instr_type) {
            let instr = architecture::Instruction { instruction: instruction.clone(), itype: architecture::InstructionType::RType };
            return Some(instr);
        }
    }
    for instr_type in &i_types {
        if opword.eq(*instr_type) {
            let instr = architecture::Instruction { instruction: instruction.clone(), itype: architecture::InstructionType::IType };
            return Some(instr);
        }
    }
    for instr_type in &j_types {
        if opword.eq(*instr_type) {
            let instr = architecture::Instruction { instruction: instruction.clone(), itype: architecture::InstructionType::JType };
            return Some(instr);
        }
    }
    for instr_type in &special_types {
        if opword.eq(*instr_type) {
            let instr = architecture::Instruction { instruction: instruction.clone(), itype: architecture::InstructionType::Special };
            return Some(instr);
        }
    }
    return None;
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
            panic!("Invalid register type! {}", register);
        }
    };
    result
}

pub fn register_name(register: u8) -> String {
    let result: &str = match register {
        0 => "$zero",
        1 => "at",
        2 => "v0",
        3 => "v1",
        4 => "a0",
        5 => "a1",
        6 => "a2",
        7 => "a3",
        8 => "t0",
        9 => "t1",
        10 => "t2",
        11 => "t3",
        12 => "t4",
        13 => "t5",
        14 => "t6",
        15 => "t7",
        16 => "s0",
        17 => "s1",
        18 => "s2",
        19 => "s3",
        20 => "s4",
        21 => "s5",
        22 => "s6",
        23 => "s7",
        24 => "t8",
        25 => "t9",
        26 => "k0",
        27 => "k1",
        28 => "gp",
        29 => "sp",
        30 => "fp",
        31 => "ra",
        _ => {
            panic!("Invalid register type! {}", register);
        }
    };
    String::from(result)
}

fn parse_immediate(imm: &String) -> i32 {
    if imm.starts_with("0x") {
        return parse_hexadecimal(imm.as_str());
    } else {
        return imm.parse::<i32>().unwrap();
    }
}

pub fn get_dest_src_imm(word: &str) -> (u8, u8, i32) {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    let dest: u8 = parse_register(components.get(1).unwrap());
    let source: u8 = parse_register(components.get(2).unwrap());
    return (dest, source, parse_immediate(components.get(3).unwrap()));
}

pub fn get_dest_imm(word: &str) -> (u8, i32) {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    let dest: u8 = parse_register(components.get(1).unwrap());
    let immediate: i32 = parse_hexadecimal(components.get(2).unwrap());
    return (dest, immediate);
}

pub fn get_rs_rt_rd(word: &str) -> (u8, u8, u8) {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    let rd: u8 = parse_register(components.get(1).unwrap());
    let rs: u8 = parse_register(components.get(2).unwrap());
    let rt: u8 = parse_register(components.get(3).unwrap());
    return (rd, rs, rt);
}

pub fn get_rd_rs_count(word: &str) -> (u8, u8, u16) {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    let rd: u8 = parse_register(components.get(1).unwrap());
    let rs: u8 = parse_register(components.get(2).unwrap());
    let count: u16 = components.get(3).unwrap().parse::<u16>().unwrap();
    return (rd, rs, count);
}

pub fn get_label(word: &str) -> String {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    components.get(1).unwrap().clone()
}

pub fn get_rt(word: &str) -> u8 {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    let target = parse_register(components.get(1).unwrap());
    return target;
}

/// Returns the register that holds the address of the memory location being read or written to.
pub fn get_memory_register(word: &str) -> (u8, u8) {
    let components: Vec<String> = word.split_whitespace().map(|s| s.to_string().replace(',', "")).collect();
    let target = parse_register(components.get(1).unwrap());
    let target2 = parse_register(components.get(2).unwrap().replace("(", "").replace(")", "").as_str());
    return (target, target2);
}

pub fn sign_extend(input: u32) -> i32 {
    ((input as i32) << 16) >> 16
}

pub fn zero_extend(input: u16) -> u32 {
    (input as u32) << 16
}