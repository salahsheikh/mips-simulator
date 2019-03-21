use std::ops::BitOr;
use std::ops::BitAnd;

use crate::parser;

#[derive(Debug)]
pub enum InstructionType {
    RType,
    IType,
    JType
}

pub struct Instruction {
    pub instruction: String,
    pub itype: InstructionType
}

impl Instruction {

    fn execute_itype(&self, processor: &mut Processor, components: Vec<String>) {
        let dest: u8 = parser::parse_register(components.get(1).unwrap());
        let source: u8 = parser::parse_register(components.get(2).unwrap());
        let immediate: u16 = parser::parse_hexadecimal(components.get(3).unwrap());
        match components.get(0).unwrap().as_str() {
            "ori" => {
                processor.set_value(dest, processor.get_value(source).bitor(immediate as u32));
            },
            _ => {
                panic!("Unhandled I-type instruction!");
            }
        }
    }

    fn execute_rtype(&self, processor: &mut Processor, components: Vec<String>) {
        let dest: u8 = parser::parse_register(components.get(1).unwrap());
        let rs: u8 = parser::parse_register(components.get(2).unwrap());
        let rt: u8 = parser::parse_register(components.get(3).unwrap());
        match components.get(0).unwrap().as_str() {
            "and" => {
                let temp = processor.get_value(rs).bitand(processor.get_value(rt));
                processor.set_value(dest, temp);
            },
            _ => {
                panic!("Unhandled R-type instruction!");
            }
        }
    }

    pub fn execute(&self, processor: &mut Processor) {
        let components: Vec<String> = self.instruction.split(" ").map(|s| s.to_string().replace(',', "")).collect();
        match self.itype {
            InstructionType::IType => {
                self.execute_itype(processor, components);
            },
            InstructionType::RType => {
                self.execute_rtype(processor, components);
            },
            InstructionType::JType => {

            }
        };
    }

}

#[derive(Debug)]
pub struct Processor {
    gpr: [u32; 32],
    pc: u32,
    hi: u32,
    lo: u32
}

impl Processor {
    /// Initialize with basic constants
    pub fn new() -> Processor {
        let mut proc = Processor {
            gpr: [0; 32],
            pc: 0x00400000,
            hi: 0x0,
            lo: 0x0
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
}