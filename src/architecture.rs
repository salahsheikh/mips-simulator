use std::ops::BitOr;
use std::ops::BitAnd;

use crate::parser;
use crate::helper;
use std::collections::HashMap;

use prettytable::Table;

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

pub enum Instructions {
    ADDI(u8, u8, i32)
}

#[allow(dead_code)]
pub struct Processor {
    gpr: [i32; 32],
    pc: u32,
    hi: u32,
    lo: u32,
    instructions: Vec<Instruction>,
    labels: HashMap<String, u32>,
    memory: [u32; 65536],
    is_running: bool
}

impl Processor {
    /// Initialize with basic constants
    pub fn new() -> Processor {
        let mut proc = Processor {
            gpr: [0; 32],
            pc: 0x00400000,
            hi: 0x0,
            lo: 0x0,
            instructions: Vec::new(),
            labels: HashMap::new(),
            memory: [0; 65536],
            is_running: true
        };
        proc.gpr[29] = 0x7fffeffc;
        proc
    }

    pub fn set_value(&mut self, destination_gpr: u8, new_value: i32) {
        if destination_gpr != 0 {
            self.gpr[destination_gpr as usize] = new_value;
        }
    }

    pub fn get_value(&self, source_gpr: u8) -> i32 {
        self.gpr[source_gpr as usize]
    }

    pub fn add_label(&mut self, label: String) {
        self.labels.insert(label.replace(':', ""), self.pc);
    }

    pub fn add_instruction(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }

    fn get_instruction(&self, address: u32) -> Option<&Instruction> {
        self.instructions.get((address - 0x00400000) as usize / 4)
    }

    fn store_word(&mut self, address: u32, word: u32) {
        self.memory[(address - 0x10010000) as usize / 4] = word;
    }

    fn load_word(&self, address: u32) -> u32 {
        self.memory[(address - 0x10010000) as usize / 4]
    }

    pub fn next(&mut self) {
        let current: Option<&Instruction> = self.get_instruction(self.pc);
        if current.is_some() {
            let instr = current.unwrap();
            let opword: &str = instr.instruction.split_whitespace().next().unwrap_or("");
            let mut branch: bool = false;
            match instr.itype {
                InstructionType::IType => {
                    match opword {
                        "ori" => {
                            let (dest, source, immediate) = parser::get_dest_src_imm(instr.instruction.as_str());
                            self.set_value(dest, self.get_value(source).bitor(immediate as i32));
                        },
                        "addi" => {
                            let (dest, source, immediate) = parser::get_dest_src_imm(instr.instruction.as_str());
                            self.set_value(dest, self.get_value(source) + immediate as i32);
                        },
                        "slti" => {
                            let (dest, source, immediate) = parser::get_dest_src_imm(instr.instruction.as_str());
                            if self.get_value(source) > immediate as i32 {
                                self.set_value(dest, 0);
                            } else {
                                self.set_value(dest, 1);
                            }
                        },
                        "sltiu" => {
                            let (dest, source, immediate) = parser::get_dest_src_imm(instr.instruction.as_str());
                            if self.get_value(source) as u32 > immediate as u32 {
                                self.set_value(dest, 0);
                            } else {
                                self.set_value(dest, 1);
                            }
                        },
                        "andi" => {
                            let (dest, source, immediate) = parser::get_dest_src_imm(instr.instruction.as_str());
                            self.set_value(dest, self.get_value(source) & immediate as i32);
                        },
                        "lui" => {
                            let (dest, immediate) = parser::get_dest_imm(instr.instruction.as_str());
                            self.set_value(dest, immediate << 16);
                        },
                        _ => {
                            panic!("Unhandled I-type instruction!");
                        }
                    }
                },
                InstructionType::RType => {
                    let (rd, rs, rt) = parser::get_rs_rt_rd(instr.instruction.as_str());
                    match opword {
                        "and" => {
                            let temp = self.get_value(rs).bitand(self.get_value(rt));
                            self.set_value(rd, temp);
                        },
                        "or" => {
                            self.set_value(rd, self.get_value(rs) & self.get_value(rt));
                        },
                        "nor" => {
                            self.set_value(rd, !(self.get_value(rs) | self.get_value(rt)));
                        },
                        "add" => {
                            self.set_value(rd, self.get_value(rs) + self.get_value(rt));
                        },
                        "sub" => {
                            self.set_value(rd, self.get_value(rs) - self.get_value(rt));
                        },
                        _ => {
                            panic!("Unhandled R-type instruction!");
                        }
                    }
                },
                InstructionType::JType => {
                    match opword {
                        "j" => {
                            let label = parser::get_label(instr.instruction.as_str());
                            if self.labels.contains_key(label.as_str()) {
                                branch = true;
                                self.pc = self.labels.get(label.as_str()).clone().unwrap().clone();
                            }
                        },
                        "jr" => {
                            self.pc = self.get_value(parser::get_rt(instr.instruction.as_str())) as u32;
                        },
                        _ => {
                            unreachable!();
                        }
                    }
                },
                InstructionType::Special => {
                    match opword {
                        "nop" => {
                            // Do nothing
                        },
                        "srl" => {
                            let (rd, rs, count) = parser::get_rd_rs_count(instr.instruction.as_str());
                            self.set_value(rd, self.get_value(rs) >> count);
                        },
                        "sll" => {
                            let (rd, rs, count) = parser::get_rd_rs_count(instr.instruction.as_str());
                            self.set_value(rd, self.get_value(rs) << count);
                        },
                        "sw" => {
                            let (source, target) = parser::get_memory_register(instr.instruction.as_str());
                            self.store_word(self.get_value(target) as u32, self.get_value(source) as u32);
                        },
                        "lw" => {
                            let (target, source) = parser::get_memory_register(instr.instruction.as_str());
                            self.set_value(target, self.load_word(self.get_value(source) as u32) as i32);
                        },
                        _ => {

                        }
                    }
                }
            };
            if !branch {
                self.pc += 4;
            }
        } else {
            self.is_running = false;
        }
    }

    pub fn is_running(&self) -> bool {
        return self.is_running;
    }

    pub fn get_instruction_count(&self) -> usize {
        self.instructions.len()
    }

    pub fn print_state(&self) {
        let mut table = Table::new();
        for i in 0..32 {
            table.add_row(row![parser::register_name(i), helper::format_as_word(self.gpr[i as usize] as u32)]);
        }
        table.add_row(row!["PC", helper::format_as_word(self.pc)]);
        table.printstd();
    }

    pub fn dump_data_memory(&self, from: u32, to: u32) {
        for address in (from..to).step_by(4) {
            println!("{}: {}", helper::format_as_word(address), helper::format_as_word(self.load_word(address)));
        }
    }

}