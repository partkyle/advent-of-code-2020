use std::fs;

extern crate lib;
use lib::error::Error;
use lib::q09::find_broken_encryption_number;

const WINDOW: usize = 25;
const FILENAME: &str = "files/09/input.txt";

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(FILENAME)?;

    let numbers: Result<Vec<u64>, _> = input.lines().map(|line| line.parse()).collect();
    let numbers = numbers?;

    let answer = find_broken_encryption_number(&numbers, WINDOW)?;

    println!("answer: {}", answer);

    Ok(())
}
