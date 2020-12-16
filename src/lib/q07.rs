use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Debug)]
pub enum GraphType {
    Ancestral,
    Generational,
}

#[derive(Debug)]
pub struct Graph<T: Ord + Copy, K: Ord + Copy> {
    nodes: BTreeSet<T>,
    adj: BTreeMap<T, BTreeSet<Node<T, K>>>,
    graph_type: GraphType,
}

impl<T: Ord + Copy, K: Ord + Copy> Graph<T, K> {
    pub fn new(graph_type: GraphType) -> Graph<T, K> {
        Graph {
            nodes: BTreeSet::new(),
            adj: BTreeMap::new(),
            graph_type: graph_type,
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.nodes.insert(node);
    }

    pub fn add_edge(&mut self, src: T, dest: T, meta: K) {
        self.add_node(src);
        self.add_node(dest);

        // decide which way to build the graph,
        // Ancestral means that the directions on the graph are going
        // towards where the bags can go inside,
        // Generation means that the directions on the graph are going
        // towards what the bags contain.
        let (src, dest) = match self.graph_type {
            GraphType::Ancestral => (dest, src),
            GraphType::Generational => (src, dest),
        };

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

    pub fn get_node(&self, item: T) -> Option<&BTreeSet<Node<T, K>>> {
        self.adj.get(&item)
    }

    pub fn visit_all(&self, item: T) -> BTreeSet<T> {
        let mut result = BTreeSet::new();
        self.visit_all_internal(&mut result, item);
        result
    }

    fn visit_all_internal(&self, result: &mut BTreeSet<T>, item: T) {
        if result.contains(&item) {
            return;
        }

        let start = self.adj.get(&item);

        if let Some(start) = start {
            for node in start.iter() {
                self.visit_all_internal(result, node.item);
                result.insert(node.item);
            }
        }
    }
}

#[derive(Eq, Debug)]
pub struct Node<T: Ord + Copy, K: Ord + Copy> {
    pub item: T,
    pub meta: K,
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

pub fn get_graph_from_sample(input: &str, graph_type: GraphType) -> Graph<&str, u8> {
    let mut g: Graph<&str, u8> = Graph::new(graph_type);

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

                    g.add_edge(bag, other_bag, count);
                }
            }
        }
    }

    g
}
