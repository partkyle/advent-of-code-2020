/*
--- Part Two ---
As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b
This list represents answers from five groups:

In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
In the second group, there is no question to which everyone answered "yes".
In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
In the fourth group, everyone answered yes to only 1 question, a.
In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?

*/
use std::convert::From;
use std::fs;

extern crate lib;
use lib::q06::CustomsSet;

const FILENAME: &str = "files/06/input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let groups = input
        // split on blank lines to get groups
        .split("\n\n")
        // convert each line of a group into a CustomsSet
        .map(|group| group.lines().map(CustomsSet::from));

    // less interesting procedural method
    // let mut total = 0;
    // for group in groups {
    //     let mut result = CustomsSet::all();
    //     for c in group {
    //         result = result.intersect(&c);
    //     }
    //     total += result.count();
    // }

    let total: usize = groups
        .map(|group| {
            group
                .fold(CustomsSet::all(), |acc, c| acc.intersect(&c))
                .count()
        })
        .sum();

    println!("answer: {}", total);
}
