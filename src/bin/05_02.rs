/*
--- Part Two ---
Ding! The "fasten seat belt" signs have turned on. Time to find your seat.

It's a completely full flight, so your seat should be the only missing boarding pass in your list. However, there's a catch: some of the seats at the very front and back of the plane don't exist on this aircraft, so they'll be missing from your list as well.

Your seat wasn't at the very front or back, though; the seats with IDs +1 and -1 from yours will be in your list.
*/

use std::collections::BTreeSet;
use std::fs;

extern crate lib;
use lib::q05::get_seat_id;

const FILENAME: &str = "files/05/input.txt";

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let seats: BTreeSet<usize> = input.lines().map(|line| get_seat_id(line)).collect();

    // only take the seats from within the ranges
    let all_seats: BTreeSet<usize> =
        (*seats.iter().min().unwrap()..*seats.iter().max().unwrap()).collect();

    let missing_seats: BTreeSet<usize> = all_seats.difference(&seats).map(|&i| i).collect();

    if missing_seats.len() != 1 {
        panic!("something went wrong, missing_seats: {:?}", missing_seats);
    }

    println!("answer: {}", missing_seats.iter().next().unwrap());
}
