use std::fs;
use std::str::FromStr;
use std::vec::Vec;

const FILENAME: &str = "files/02/input.txt";
/*
--- Day 2: Password Philosophy ---
Your flight departs in a few days from the coastal airport; the easiest way down to the coast from here is via toboggan.

The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day. "Something's wrong with our computers; we can't log in!" You ask if you can take a look.

Their password database seems to be a little corrupted: some of the passwords wouldn't have been allowed by the Official Toboggan Corporate Policy that was in effect when they were chosen.

To try to debug the problem, they have created a list (your puzzle input) of passwords (according to the corrupted database) and the corporate policy when that password was set.

For example, suppose you have the following list:

1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
Each line gives the password policy and then the password. The password policy indicates the lowest and highest number of times a given letter must appear for the password to be valid. For example, 1-3 a means that the password must contain a at least 1 time and at most 3 times.

In the above example, 2 passwords are valid. The middle password, cdefg, is not; it contains no instances of b, but needs at least 1. The first and third passwords are valid: they contain one a or nine c, both within the limits of their respective policies.

How many passwords are valid according to their policies?
*/

#[derive(Debug)]
struct Item {
    min: u8,
    max: u8,
    letter: String,
    password: String,
}

impl Item {
    pub fn valid(&self) -> bool {
        let count = self.password.matches(&self.letter).count();
        // TODO: look up number type conversions and see what the most elegant way is
        count >= self.min.into() && count <= self.max.into()
    }
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_split: Vec<&str> = s.split(":").collect();

        if colon_split.len() != 2 {
            return Err("invalid parse. Wrong number of elements past `:`".to_string());
        }

        let range_letter_split: Vec<&str> = colon_split[0].split(" ").collect();

        if range_letter_split.len() != 2 {
            return Err("invalid parse. Wrong number of elements past `-`".to_string());
        }

        let min_max: Result<Vec<u8>, _> = range_letter_split[0]
            .split("-")
            .map(|e| e.parse())
            .collect();

        let (min, max) = match min_max {
            Err(err) => return Err(format!("could not get min and max: {}", err)),
            Ok(min_max) => (min_max[0], min_max[1]),
        };

        Ok(Item {
            min: min,
            max: max,
            letter: range_letter_split[1].trim().to_string(),
            password: colon_split[1].trim().to_string(),
        })
    }
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let item_result: Result<Vec<Item>, _> = input.lines().map(Item::from_str).collect();
    let items = item_result.expect("error parsing lines");

    let p = items.iter().filter(|e| e.valid()).count();

    println!("answer: {:?}", p);
}
