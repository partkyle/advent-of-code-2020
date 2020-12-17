use std::collections::HashSet;

pub fn find_pair(items: HashSet<i64>, n: i64) -> HashSet<i64> {
    let diff: HashSet<i64> = items.iter().map(|e| n - e).collect();
    items.intersection(&diff).map(|&e| e).collect()
}
