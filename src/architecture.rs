use std::ops::BitOr;
use std::ops::BitAnd;

use crate::parser;

#[derive(Debug)]
pub enum InstructionType {
    RType,
    IType,
    JType,
    Special
}

pub struct Instruction {
    pub instruction: String,
    pub itype: InstructionType
}

pub struct Processor {
    gpr: [u32; 32],
    pc: u32,
    hi: u32,
    lo: u32,
    instructions: Vec<Instruction>
}

impl Processor {
    /// Initialize with basic constants
    pub fn new() -> Processor {
        let mut proc = Processor {
            gpr: [0; 32],
            pc: 0x00400000,
            hi: 0x0,
            lo: 0x0,
            instructions: Vec::new()
        };
        proc.gpr[29] = 0x7fffeffc;
        proc
    }

    pub fn set_value(&mut self, destination_gpr: u8, new_value: u32) {
        if destination_gpr != 0 {
            self.gpr[destination_gpr as usize] = new_value;
        }
    }

    pub fn get_value(&self, source_gpr: u8) -> u32 {
        self.gpr[source_gpr as usize]
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    fn get_instruction(&self, address: u32) -> &Instruction {
        self.instructions.get((address - 0x00400000) as usize / 4).unwrap()
    }

    pub fn next(&mut self) {
        let current: &Instruction = self.get_instruction(self.pc);
        let components: Vec<String> = current.instruction.split(" ").map(|s| s.to_string().replace(',', "")).collect();
        let mut branch: bool = false;
        match current.itype {
            InstructionType::IType => {
                let dest: u8 = parser::parse_register(components.get(1).unwrap());
                let source: u8 = parser::parse_register(components.get(2).unwrap());
                let immediate: u16 = parser::parse_hexadecimal(components.get(3).unwrap());
                match components.get(0).unwrap().as_str() {
                    "ori" => {
                        self.set_value(dest, self.get_value(source).bitor(immediate as u32));
                    },
                    _ => {
                        panic!("Unhandled I-type instruction!");
                    }
                }
            },
            InstructionType::RType => {
                let dest: u8 = parser::parse_register(components.get(1).unwrap());
                let rs: u8 = parser::parse_register(components.get(2).unwrap());
                let rt: u8 = parser::parse_register(components.get(3).unwrap());
                match components.get(0).unwrap().as_str() {
                    "and" => {
                        let temp = self.get_value(rs).bitand(self.get_value(rt));
                        self.set_value(dest, temp);
                    },
                    _ => {
                        panic!("Unhandled R-type instruction!");
                    }
                }
            },
            InstructionType::JType => {

            },
            InstructionType::Special => {
                match components.get(0).unwrap().as_str() {
                    "nop" => {
                        // Do nothing
                    },
                    _ => {

                    }
                }
            }
        };
        if branch {
            // branching code
        } else {
            self.pc += 4;
        }
    }

    pub fn get_instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn print_state(&self) {
        println!("{:?}\n {}, {}, {:X}", self.gpr, self.hi, self.lo, self.pc);
    }
}