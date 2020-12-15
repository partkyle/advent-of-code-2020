use std::convert::From;
use std::fmt;

#[derive(Clone, Debug)]
pub struct CustomsSet {
    bits: u64,
}

impl CustomsSet {
    pub fn new() -> CustomsSet {
        CustomsSet { bits: 0 }
    }

    pub fn all() -> CustomsSet {
        "abcdefghijklmnopqrstuvwxyz".into()
    }

    fn within_range(i: u8) -> bool {
        // u8 cannot be negative
        i < 26
    }

    pub fn count(&self) -> usize {
        self.to_string().len()
    }

    pub fn union(&self, other: &CustomsSet) -> CustomsSet {
        let mut c = self.clone();
        c.bits |= other.bits;
        c
    }

    pub fn intersect(&self, other: &CustomsSet) -> CustomsSet {
        let mut c = self.clone();
        c.bits &= other.bits;
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
fn test_union() {
    let cases = [
        ("abc", "def", "abcdef"),
        ("abcd", "abf", "abcdf"),
        ("ab123de", "", "abde"),
    ];

    for &(in_a, in_b, expected) in cases.iter() {
        let a: CustomsSet = in_a.into();
        let b: CustomsSet = in_b.into();
        let result = a.union(&b);
        assert_eq!(result.to_string(), expected)
    }
}

#[test]
fn test_intersect() {
    let cases = [
        ("abc", "def", ""),
        ("abcd", "abf", "ab"),
        ("ab123de", "", ""),
        ("ab123de", "dea", "ade"),
    ];

    for &(in_a, in_b, expected) in cases.iter() {
        let a: CustomsSet = in_a.into();
        let b: CustomsSet = in_b.into();
        let result = a.intersect(&b);
        assert_eq!(result.to_string(), expected)
    }
}

#[test]
fn test_from_str() {
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
