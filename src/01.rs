use std::collections::HashSet;
use std::fs;

const LEDGER: &str = "1721
979
366
299
675
1456";

const FILENAME: &str = "files/input-1.txt";

fn main() {
    // let input = LEDGER.clone();
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let items: HashSet<i64> = input.lines().map(|e| e.parse().unwrap()).collect();
    let diff: HashSet<i64> = items.iter().map(|e| 2020 - e).collect();

    let intersection: HashSet<_> = items.intersection(&diff).collect();

    if intersection.len() == 2 {
        // got em
        let p = intersection.iter().fold(1, |acc, &e| acc * e);
        println!("answer: {}", p)
    } else {
        println!("got unexpected intersection {:?}", intersection);
    }
}
