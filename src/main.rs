use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub mod architecture;
pub mod parser;

fn main() {
    let mut instructions: Vec<architecture::Instruction> = Vec::new();
    let f = File::open("./src.asm");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };


    let reader = BufReader::new(f);
    for line in reader.lines() {
        let instr: String = line.unwrap();
        if !instr.is_empty() {
            instructions.push(parser::parse_function(instr).unwrap());
        }
    }

    let mut proc: architecture::Processor = architecture::Processor::new();

    for i in instructions {
        i.execute(&mut proc);
    }
    println!("{:?}", proc);
}
