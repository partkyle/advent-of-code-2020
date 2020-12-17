use std::convert::TryInto;
use std::fs;

extern crate lib;
use lib::q08::{line_to_instruction, run_instructions, Instruction};

const FILENAME: &str = "files/08/input.txt";

struct ModInstructionsIter {
    instructions: Vec<Instruction>,
    iptr: usize,
}

impl ModInstructionsIter {
    pub fn new(instructions: Vec<Instruction>) -> ModInstructionsIter {
        ModInstructionsIter {
            instructions: instructions,
            iptr: 0,
        }
    }
}

impl Iterator for ModInstructionsIter {
    type Item = Vec<Instruction>;
    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Vec<Instruction>> {
        if self.iptr >= self.instructions.len().try_into().unwrap() {
            return None;
        }

        let mut instructions = self.instructions.clone();
        while self.iptr < instructions.len().try_into().unwrap() {
            match instructions[self.iptr] {
                Instruction::JMP(n) => {
                    instructions[self.iptr] = Instruction::NOP(n);
                    break;
                }

                Instruction::NOP(n) => {
                    instructions[self.iptr] = Instruction::JMP(n);
                    break;
                }

                _ => {}
            }

            self.iptr += 1;
        }

        // always advance 1 because we are going to go on the next iteration.
        self.iptr += 1;

        Some(instructions)
    }
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("failed to read file");

    let instructions: Option<Vec<Instruction>> = input.lines().map(line_to_instruction).collect();

    if let Some(instructions) = instructions {
        for instructions in ModInstructionsIter::new(instructions) {
            let acc = run_instructions(instructions, false);
            match acc {
                Some(acc) => {
                    println!("answer: {}", acc);
                    return;
                }
                None => {}
            }
        }
    }

    println!("nothing was found");
}
