use std::collections::HashSet;

use crate::error::Error;
use crate::q01::find_pair;

pub fn find_broken_encryption_number(numbers: &Vec<u64>, window: usize) -> Result<u64, Error> {
    for (i, &n) in numbers[window..].iter().enumerate() {
        let mut set: HashSet<u64> = HashSet::new();
        set.extend(&numbers[i..i + window]);

        let pair = find_pair(set.iter().map(|&i| i as i64).collect(), n as i64);

        if pair.len() == 0 {
            return Ok(n);
        }
    }

    Err("not found".into())
}
