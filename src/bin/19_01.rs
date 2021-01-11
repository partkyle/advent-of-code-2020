use std::collections::{BTreeMap, BTreeSet};
use std::fs;

extern crate lib;
use lib::error::Error;

const FILENAME: &str = "files/19/sample.txt";

#[derive(Debug)]
enum Rule {
    Redirection(Vec<Vec<usize>>),
    Char(String),
}

// parse the ruleset from an iterator of lines
// this is going in include the empty newline and values to compare later
// so it needs to handle that case and break early.
fn parse_ruleset<'a, I>(mut lines: I) -> Result<BTreeMap<usize, Rule>, Error>
where
    I: Iterator<Item = &'a str>,
{
    let mut rules = BTreeMap::new();

    // the first chunk of the file is the list of rules
    while let Some(line) = lines.next() {
        let rule_parts: Vec<&str> = line.split(": ").collect();

        // skip if there is a newline, which will throw away the newline,
        // leaving the iterator only having the dataset left
        if line == "" {
            break;
        }

        // sanity check error, this never occurs
        if rule_parts.len() != 2 {
            return Err("line part not correct".into());
        }

        let rule_no: usize = rule_parts[0].parse()?;

        // figure out what type of rule this is
        let rule_line = rule_parts[1];
        if rule_line.contains("\"") {
            // the input is all single quoted strings, so we will simply remove strings
            // and use what's left.
            let ch = rule_line.replace("\"", "");
            rules.insert(rule_no, Rule::Char(ch));
        } else {
            // Treat all Redirection types as the same. Split on the " | " to gather the separate sets of rules
            let rule_line_parsed: Result<Vec<Vec<usize>>, _> = rule_line
                .split(" | ")
                .map(|s| {
                    s.split_whitespace()
                        .map(|i| i.parse::<usize>())
                        .collect::<Result<Vec<usize>, _>>()
                })
                .collect();
            rules.insert(rule_no, Rule::Redirection(rule_line_parsed?));
        }
    }

    Ok(rules)
}

// fn expand_rule(n: usize, rules: &BTreeMap<usize, Rule>) -> Result<Vec<String>, Error> {
//     let start = rules.get(&n).expect("none found");

//     match start {
//         Rule::Redirection(sets) => {
//             let mut results = vec![""];
//             for set in sets {
//                 for &n in set {
//                     for expanded in expand_rule(n, &rules)? {
//                         let result = "".to_string();
//                         results.iter().map(|s| s + expanded)
//                     }
//                 }
//             }

//             return Ok(results);
//         }
//         Rule::Char(s) => return Ok(vec![s.to_string()]),
//     }
// }

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(FILENAME)?;

    let mut line_iter = input.lines();

    let rules = parse_ruleset(&mut line_iter)?;

    println!("{:?}", rules);

    // let expanded = expand_rule(0, &rules);
    // println!("{:?}", expanded);

    // let's try creating all allowable strings based on rule 0
    // and storing them in a set
    // this might not work for large datasets

    // the remainder of the lines to be compared
    while let Some(line) = line_iter.next() {
        println!("{:?}", line);
    }

    Ok(())
}
