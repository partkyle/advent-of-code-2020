use std::collections::HashSet;
use std::fs;
use std::u64;

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

    for slider_size in 2..numbers.len() {
        for i in 0..(numbers.len() - slider_size) {
            let window = &numbers[i..i + slider_size];

            let mut min = u64::MAX;
            let mut max = u64::MIN;
            let mut total = 0;

            let mut nums = HashSet::new();

            for &n in window.iter() {
                nums.insert(n);
                total += n;
                if n < min {
                    min = n;
                }
                if max < n {
                    max = n;
                }
            }

            if total == answer {
                println!("answer: {}", min + max);
                return Ok(());
            }
        }
    }

    Ok(())
}
