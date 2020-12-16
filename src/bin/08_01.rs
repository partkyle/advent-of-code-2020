use std::fs;

extern crate lib;
use lib::q08::{line_to_instruction, run_instructions, Instruction};

const FILENAME: &str = "files/08/input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("failed to read file");

    let instructions: Option<Vec<Instruction>> = input.lines().map(line_to_instruction).collect();

    if let Some(instructions) = instructions {
        let acc = run_instructions(instructions, true);
        println!("answer: {}", acc.unwrap());
    }
}
