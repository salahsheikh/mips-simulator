use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub mod architecture;
pub mod helper;
pub mod parser;

extern crate clap;
use clap::{Arg, App};

#[macro_use] extern crate prettytable;

fn main() {
    let matches = App::new("MIPS Simulator")
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
    }

    proc.print_state();

    proc.dump_data_memory(0x10010000, 0x1001000c);
}
