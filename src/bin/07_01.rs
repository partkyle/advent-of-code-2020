/*
--- Day 7: Handy Haversacks ---
You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

```
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
```

These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

```
A bright white bag, which can hold your shiny gold bag directly.
A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
```

So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
*/

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug)]
struct Graph<T: Ord + Copy, K: Ord + Copy> {
    nodes: BTreeSet<T>,
    adj: BTreeMap<T, BTreeSet<Node<T, K>>>,
}

impl<T: Ord + Copy, K: Ord + Copy> Graph<T, K> {
    pub fn new() -> Graph<T, K> {
        Graph {
            nodes: BTreeSet::new(),
            adj: BTreeMap::new(),
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, src: T, dest: T, meta: K) {
        self.add_node(src);
        self.add_node(dest);

        match self.adj.get_mut(&src) {
            Some(m) => {
                m.insert(Node::new(dest, meta));
            }
            None => {
                let mut set = BTreeSet::new();
                set.insert(Node::new(dest, meta));
                self.adj.insert(src, set);
            }
        };
    }

    pub fn visit_all_parents(&self, item: T) -> BTreeSet<T> {
        let mut result = BTreeSet::new();
        self.visit_all_parents_internal(&mut result, item);
        result
    }

    fn visit_all_parents_internal(&self, result: &mut BTreeSet<T>, item: T) {
        if result.contains(&item) {
            return;
        }

        let start = self.adj.get(&item);

        if let Some(start) = start {
            for node in start.iter() {
                self.visit_all_parents_internal(result, node.item);
                result.insert(node.item);
            }
        }
    }
}

#[derive(Eq, Debug)]
struct Node<T: Ord + Copy, K: Ord + Copy> {
    item: T,
    meta: K,
}

impl<T: Ord + Copy, K: Ord + Copy> Node<T, K> {
    pub fn new(item: T, meta: K) -> Node<T, K> {
        Node {
            item: item,
            meta: meta,
        }
    }
}

impl<T: Ord + Copy, K: Ord + Copy> PartialEq for Node<T, K> {
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<T: Ord + Copy, K: Ord + Copy> PartialOrd for Node<T, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.item.partial_cmp(&other.item)
    }
}

impl<T: Ord + Copy, K: Ord + Copy> Ord for Node<T, K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.item.cmp(&other.item)
    }
}

use std::fs;

const FILENAME: &str = "files/07/input.txt";

fn get_graph_from_sample(input: &str) -> Graph<&str, u8> {
    let mut g: Graph<&str, u8> = Graph::new();

    for line in input.lines() {
        let no_punc = &line[..line.len() - 1];
        let parts: Vec<&str> = no_punc.split(" contain ").collect();

        if parts.len() != 2 {
            panic!("wrong parts")
        }

        let bag = &parts[0][..parts[0].len() - " bags".len()];

        match parts[1] {
            "no other bags" => g.add_node(bag),
            _ => {
                for bag_ref in parts[1].split(", ") {
                    let bag_parts: Vec<&str> = bag_ref.splitn(2, " ").collect();
                    if bag_parts.len() != 2 {
                        panic!("wrong bag_parts");
                    }
                    let other_bag = &bag_parts[1][..]
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag");
                    let count: u8 = bag_parts[0].parse().unwrap_or(0);
                    g.add_edge(other_bag, bag, count);
                }
            }
        }
    }

    g
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("failed to read file");

    let g = get_graph_from_sample(&input);

    let result = g.visit_all_parents("shiny gold");
    println!("answer: {}", result.len());
}
