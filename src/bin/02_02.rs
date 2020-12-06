/*
--- Part Two ---
While it appears you validated the passwords correctly, they don't seem to be what the Official Toboggan Corporate Authentication System is expecting.

The shopkeeper suddenly realizes that he just accidentally explained the password policy rules from his old job at the sled rental place down the street! The Official Toboggan Corporate Policy actually works a little differently.

Each policy actually describes two positions in the password, where 1 means the first character, 2 means the second character, and so on. (Be careful; Toboggan Corporate Policies have no concept of "index zero"!) Exactly one of these positions must contain the given letter. Other occurrences of the letter are irrelevant for the purposes of policy enforcement.

Given the same example list from above:

1-3 a: abcde is valid: position 1 contains a and position 3 does not.
1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
How many passwords are valid according to the new interpretation of the policies?

*/

use std::fs;
use std::vec::Vec;

extern crate lib;
use lib::q02::Item;
use std::str::FromStr;

const FILENAME: &str = "files/02/input.txt";

fn valid(item: &Item) -> bool {
    let indexes = [item.min, item.max];
    // get the characters at the min/max indexes and store that in a vector
    // TODO: figure out error handling here in a way that doesn't require `unwrap`
    let chars: Vec<String> = indexes
        .iter()
        .map(|i| {
            item.password
                .chars()
                .nth((i - 1).into())
                .unwrap()
                .to_string()
        })
        .collect();

    // TODO: I don't understand the references and whatnot here
    chars.iter().filter(|&c| *c == item.letter).count() == 1
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let item_result: Result<Vec<Item>, _> = input.lines().map(Item::from_str).collect();
    let items = item_result.expect("error parsing lines");

    let p = items.into_iter().filter(valid).count();

    println!("answer: {:?}", p);
}
