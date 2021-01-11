use std::fs;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

extern crate lib;
use lib::error::Error;

#[derive(Parser)]
#[grammar = "../files/19/input_pt2.pest"]
struct IdentParser;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("files/19/input_pt2_data.txt")?;

    let count = input
        .lines()
        .map(|line| IdentParser::parse(Rule::PROGRAM, line))
        .filter(Result::is_ok)
        .count();

    println!("{}", count);

    Ok(())
}
