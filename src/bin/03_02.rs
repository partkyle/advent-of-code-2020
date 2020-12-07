/*

--- Part Two ---

Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

Right 1, down 1.
Right 3, down 1. (This is the slope you already checked.)
Right 5, down 1.
Right 7, down 1.
Right 1, down 2.
In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/
use std::fs;

extern crate lib;
use lib::q03::Board;

const FILENAME: &str = "files/03/input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let board = Board::new(input).expect("failed to create board from input file");

    /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
    */
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mulitplied = slopes
        .iter()
        .map(|s| board.count_trees_from_slope(s.0, s.1))
        .fold(1, |acc, n| acc * n);

    println!("answer: {}", mulitplied);
}
