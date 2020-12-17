use std::collections::HashSet;
use std::fs;

extern crate lib;
use lib::error::Error;
use lib::q01::find_pair;

const WINDOW: usize = 25;
const FILENAME: &str = "files/09/input.txt";

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(FILENAME)?;

    let numbers: Result<Vec<u64>, _> = input.lines().map(|line| line.parse()).collect();
    let numbers = numbers?;

    for (i, &n) in numbers[WINDOW..].iter().enumerate() {
        let mut set: HashSet<u64> = HashSet::new();
        set.extend(&numbers[i..i + WINDOW]);

        let pair = find_pair(set.iter().map(|&i| i as i64).collect(), n as i64);

        if pair.len() == 0 {
            println!("answer: {}", n);
            return Ok(());
        }
    }

    Ok(())
}
