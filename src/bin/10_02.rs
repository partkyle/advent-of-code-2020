use std::fs;

extern crate lib;
use lib::error::Error;

const FILENAME: &str = "files/10/input.txt";

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(FILENAME)?;

    let numbers: Result<Vec<u64>, _> = input.lines().map(|line| line.parse()).collect();
    let mut numbers = numbers?;
    numbers.sort();
    numbers.push(numbers[numbers.len() - 1] + 3);
    let numbers = numbers;

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

    // find contiguous blocks of 1
    // takes advantage of the fact that they are all 1 or 3 differences
    // ignores the 1 on the boundary of the 3 because that would cause a difference of 4
    let mut blocks = vec![];
    let mut current_block = vec![];
    for (i, &n) in differences.iter().enumerate() {
        if n == 1 {
            current_block.push(i);
        } else {
            // at the boundary, the last one cannot be removed.
            if current_block.len() > 1 {
                current_block.remove(current_block.len() - 1);
                blocks.push(current_block);
            }

            current_block = vec![];
        }
    }

    let product = blocks
        .iter()
        .map(|b| match b.len() {
            3 => 7,
            2 => 4,
            1 => 2,
            _ => panic!("there are no other numbers supported"),
        })
        .fold(1u64, |acc, n| acc * n);

    println!("answer: {}", product);

    Ok(())
}



/*

block [0, 1, 2]

st=> (0, 0)

0 => (1, 1)
1 => (2, 1)
2 => (3, 1)

3 => (4, 1)
4 => (7, 3)


210
---
000 x
001
010
011
100
101
110
111

============================

block [6, 7]

5 => (10, 3)

6 => (11, 1)
7 => (12, 1)

8 => (13, 1)
9 => (16, 3)

10
--
00
01
10
11

===============================

block [20]

19 => (32, 3)

20 => (33, 1)

21 => (34, 1)

0
-
0
1
*/
