use crate::parser;

#[derive(Debug)]
pub enum InstructionType {
    RType,
    IType,
    JType
}

pub struct Instruction {
    instruction: String,
    itype: InstructionType
}

impl Instruction {
    fn execute(&self, processor: &mut Processor) {
        let components: Vec<String> = self.instruction.split(" ").map(|s| s.to_string().replace(',', "")).collect();
        match self.itype {
            InstructionType::IType => {
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
            },
            InstructionType::RType => {

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