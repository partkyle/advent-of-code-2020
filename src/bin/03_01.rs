/*
--- Day 3: Toboggan Trajectory ---
With the toboggan login problems resolved, you set off toward the airport.
While travel by toboggan might be easy, it's certainly not safe: there's very
minimal steering and the area is covered in trees. You'll need to see which
angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer
coordinates in a grid. You make a map (your puzzle input) of the open
squares (.) and trees (#) you can see. For example:

..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#

These aren't the only trees, though; due to something you read about once
involving arboreal genetics and biome stability, the same pattern repeats
to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

You start on the open square (.) in the top-left corner and need to reach
the bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper model
that prefers rational numbers); start by counting all the trees you would
encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right
3 and down 1. Then, check the position that is right 3 and down 1 from there,
and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where
there was an open square and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->

In this example, traversing the map using this slope would cause you to
encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 3
and down 1, how many trees would you encounter?

*/

use std::fs;
use std::str::FromStr;

const FILENAME: &str = "files/03/input.txt";

#[derive(Debug, Clone)]
enum Space {
    Empty,
    Tree,
}

impl FromStr for Space {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Space::Empty),
            "#" => Ok(Space::Tree),
            _ => Err(format!("Not a board type: {}", s)),
        }
    }
}

struct Board<Space> {
    board: Vec<Vec<Space>>,
}

impl Board<Space> {
    fn new(input: String) -> Result<Board<Space>, String> {
        let board_result: Result<Vec<Vec<Space>>, _> = input
            .lines()
            .map(|s| s.chars().map(|c| c.to_string().parse()).collect())
            .collect();

        match board_result {
            Err(e) => Err(format!("error creating board state: {}", e)),
            Ok(result) => Ok(Board { board: result }),
        }
    }

    fn get_at(&self, x: usize, y: usize) -> Space {
        self.board[y][x].clone()
    }

    fn get_at_wrapping(&self, x: usize, y: usize) -> Space {
        let new_x = x % self.board[y].len();
        self.get_at(new_x, y)
    }
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let board = Board::new(input).unwrap();

    let (mut x, mut y) = (0, 0);
    let mut total = 0;
    while y < board.board.len() {
        total += match board.get_at_wrapping(x, y) {
            Space::Tree => 1,
            Space::Empty => 0,
        };
        x += 3;
        y += 1;
    }

    println!("answer: {}", total);
}
