use std::collections::HashSet;
use std::fs;

extern crate lib;
use lib::q01::find_pair;

const FILENAME: &str = "files/01/input.txt";

fn main() {
    // let input = LEDGER.clone();
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let items: HashSet<i64> = input.lines().map(|e| e.parse().unwrap()).collect();

    for &item in items.iter() {
        // create sublist exluding this item iteration
        let mut copy_items = items.clone();
        copy_items.remove(&item);

        // remove the value of what we are looking for from all in the sublist
        // let new_items: HashSet<_> = items.iter().map(|e| e).collect();

        // using the algorithm from part 1, find a new total, where with the item they will add up to 2020
        // account for item twice because it's removed from both other items

        /*
        a + b = (2020 - item)
        + item  + item
        --------------
        a + b + item = 2020
        */
        let intersect = find_pair(copy_items, 2020 - item);
        if intersect.len() == 2 {
            // remember to readd the item value here so we can find it
            let p = item * intersect.iter().fold(1, |acc, &e| acc * e);
            println!("answer: {}", p);
            return;
        }
    }

    println!("no answer found");
}
