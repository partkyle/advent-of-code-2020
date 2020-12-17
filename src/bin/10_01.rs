use std::fs;

extern crate lib;
use lib::error::Error;

const FILENAME: &str = "files/10/input.txt";

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(FILENAME)?;

    let numbers: Result<Vec<u64>, _> = input.lines().map(|line| line.parse()).collect();
    let mut numbers = numbers?;

    numbers.sort();

    // calculate the differences using the previous number
    let mut prev = 0u64;
    let differences: Vec<u64> = numbers
        .iter()
        .map(|&n| {
            let res = n - prev;
            prev = n;
            res
        })
        .collect();

    let (ones, threes) = differences.iter().fold((0u64, 0u64), |mut acc, &n| {
        if n == 1 {
            acc.0 += 1;
        } else if n == 3 {
            acc.1 += 1;
        }
        acc
    });

    // need to account for the last adaptor (the phone)
    // which will be 3 different from the highest one.
    println!("answer: {}", ones * (threes + 1));

    Ok(())
}
