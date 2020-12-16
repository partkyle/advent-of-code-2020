use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug)]
pub enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

impl Instruction {
    pub fn iptr(&self) -> i64 {
        match self {
            Instruction::JMP(i) => *i,
            _ => 1,
        }
    }

    pub fn acc(&self) -> i64 {
        match self {
            Instruction::ACC(i) => *i,
            _ => 0,
        }
    }
}

pub fn line_to_instruction(s: &str) -> Option<Instruction> {
    let lower = s.to_lowercase();
    let sp: Vec<&str> = lower.splitn(2, " ").collect();

    let num: Result<i64, _> = sp[1].parse();

    if let Ok(num) = num {
        return match sp[0] {
            "nop" => Some(Instruction::NOP(num)),
            "acc" => Some(Instruction::ACC(num)),
            "jmp" => Some(Instruction::JMP(num)),
            _ => None,
        };
    }

    None
}

pub fn run_instructions(instructions: Vec<Instruction>, early_exit: bool) -> Option<i64> {
    let mut iptr = 0i64;
    let mut acc = 0i64;

    let mut instruction_tracker = HashSet::new();

    while iptr < instructions.len().try_into().unwrap() {
        if instruction_tracker.contains(&iptr) {
            break;
        }

        instruction_tracker.insert(iptr);

        let current = &instructions[iptr as usize];

        iptr += current.iptr();
        acc += current.acc();
    }

    if !early_exit {
        if iptr != instruction_tracker.len().try_into().unwrap() {
            return None;
        }
    }

    Some(acc)
}
