use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub mod architecture;
pub mod parser;

fn main() {
    let mut proc: architecture::Processor = architecture::Processor::new();

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
            if instr.contains(':') {
                proc.add_label(instr);
            } else {
                proc.add_instruction(parser::parse_function(instr).unwrap());
            }
        }
    }

    proc.next();
    proc.print_state();
}
