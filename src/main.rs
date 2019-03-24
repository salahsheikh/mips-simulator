use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub mod architecture;
pub mod parser;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("MIPS Simulator")
        .version("0.1")
        .author("Salah Sheikh <salahsheikh AT outlook DOT com>")
        .arg(Arg::with_name("input")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    let mut proc: architecture::Processor = architecture::Processor::new();

    let f = File::open(matches.value_of("input").unwrap());
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

    while proc.is_running() {
        proc.next();
        proc.print_state();
    }
}