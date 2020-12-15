/*--- Day 6: Custom Customs ---
As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

```
abcx
abcy
abcz
```

In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:

```
abc

a
b
c

ab
ac

a
a
a
a

b
```

This list represents answers from five groups:

The first group contains one person who answered "yes" to 3 questions: a, b, and c.
The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
The last group contains one person who answered "yes" to only 1 question, b.
In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?

*/
use std::convert::From;
use std::fmt;
use std::fs;

const FILENAME: &str = "files/06/input.txt";

#[derive(Clone)]
struct CustomsSet {
    bits: u64,
}

impl CustomsSet {
    pub fn new() -> CustomsSet {
        CustomsSet { bits: 0 }
    }

    fn within_range(i: u8) -> bool {
        // u8 cannot be negative
        i < 26
    }

    pub fn count(&self) -> usize {
        self.to_string().len()
    }

    pub fn merge(&self, other: &CustomsSet) -> CustomsSet {
        let mut c = self.clone();
        c.bits |= other.bits;
        c
    }

    pub fn set(&mut self, bit: u8) {
        if !CustomsSet::within_range(bit) {
            return;
        }
        self.bits |= 1 << bit;
    }

    pub fn set_char(&mut self, bit: char) {
        // TODO: adjust to not cheat on using 255 as a failure condition
        let v = (bit as u8).checked_sub('a' as u8).unwrap_or(255);

        self.set(v);
    }
}

impl fmt::Display for CustomsSet {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.

        let s: String = (0..26)
            .filter(|letter| self.bits & 1 << letter > 0)
            .map(|i| (i + 'a' as u8) as char)
            .collect();

        write!(f, "{}", s)
    }
}

impl From<&str> for CustomsSet {
    fn from(s: &str) -> Self {
        let mut set = CustomsSet::new();

        for c in s.chars() {
            set.set_char(c);
        }

        set
    }
}
#[test]
fn test_count() {
    let cases = [("abc", 3), ("abcd", 4), ("ab123de", 4)];

    for &(input, expected) in cases.iter() {
        let a: CustomsSet = input.into();
        assert_eq!(a.count(), expected)
    }
}

#[test]
fn test_merge() {
    let cases = [
        ("abc", "def", "abcdef"),
        ("abcd", "abf", "abcdf"),
        ("ab123de", "", "abde"),
    ];

    for &(in_a, in_b, expected) in cases.iter() {
        let a: CustomsSet = in_a.into();
        let b: CustomsSet = in_b.into();
        let result = a.merge(&b);
        assert_eq!(result.to_string(), expected)
    }
}

#[test]
fn test_customset_from_str() {
    let cases = [
        ("abc", "abc"),
        // they can have newlines
        ("a\nb\nc", "abc"),
        ("a&dh2^abe", "abdeh"),
        ("123145431231", ""),
    ];

    for &(input, expected) in cases.iter() {
        let set: CustomsSet = input.into();
        assert_eq!(format!("{}", set), expected);
    }
}

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let groups = input.split("\n\n");

    let customs_groups: Vec<CustomsSet> = groups.map(|group| group.into()).collect();

    let answer: usize = customs_groups.iter().map(|c| c.count()).sum();
    println!("answer: {}", answer);
}
