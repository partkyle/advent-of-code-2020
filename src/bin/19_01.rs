use std::collections::BTreeMap;
use std::fs;

extern crate regex;
use regex::Regex;

extern crate lib;
use lib::error::Error;

const FILENAME: &str = "files/19/input.txt";

#[derive(Debug)]
enum Rule {
    Simple(Vec<usize>),
    LeftRight(Box<Rule>, Box<Rule>),
    Char(String),
}

struct World<'a> {
    start: usize,
    rules: &'a BTreeMap<usize, Rule>,
}

impl<'a> World<'a> {
    fn new(start: usize, rules: &'a BTreeMap<usize, Rule>) -> World {
        World {
            start: start,
            rules: rules,
        }
    }

    // fully expand a rule from a list of ids to a regex to use for parsing
    pub fn regex(&self) -> String {
        let start = self.rules.get(&self.start).unwrap();

        let mut result = "^".to_string();
        result += &self.regex_internal(start);
        result += "$";

        result
    }

    fn regex_internal(&self, rule: &Rule) -> String {
        match rule {
            Rule::Simple(list) => {
                let mut result = "".to_string();
                for i in list {
                    let r = self.rules.get(&i).unwrap();
                    result += &self.regex_internal(r);
                }
                result
            }

            Rule::LeftRight(left, right) => {
                format!(
                    "({}|{})",
                    self.regex_internal(left),
                    self.regex_internal(right)
                )
            }

            Rule::Char(s) => format!("{}", s),
        }
    }
}

fn parse_simple(rule_line: &str) -> Result<Vec<usize>, Error> {
    let ids: Result<Vec<usize>, _> = rule_line.split_whitespace().map(|i| i.parse()).collect();
    let ids = ids?;
    Ok(ids)
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
            if rule_line.contains(" | ") {
                let parts: Vec<&str> = rule_line.split(" | ").collect();

                if parts.len() != 2 {
                    return Err("wrong number of split parts in the | operator".into());
                }

                rules.insert(
                    rule_no,
                    Rule::LeftRight(
                        Box::new(Rule::Simple(parse_simple(parts[0])?)),
                        Box::new(Rule::Simple(parse_simple(parts[1])?)),
                    ),
                );
            // TODO: this only handles 2
            } else {
                rules.insert(rule_no, Rule::Simple(parse_simple(rule_line)?));
            }
        }
    }

    Ok(rules)
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string(FILENAME)?;

    let mut line_iter = input.lines();

    let rules = parse_ruleset(&mut line_iter)?;

    let world = World::new(0, &rules);

    let re = Regex::new(&world.regex()).map_err(|_e| "nothing")?;

    let count = line_iter.filter(|line| re.is_match(line)).count();
    println!("answer: {}", count);

    Ok(())
}
