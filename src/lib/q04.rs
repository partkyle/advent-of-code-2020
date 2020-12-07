use std::collections::HashMap;
use std::convert::{From, Into, TryFrom};
use std::usize;

fn valid_year(s: &str, min: usize, max: usize) -> bool {
    let yr: usize = s.parse().unwrap_or(0);
    yr >= min && yr <= max
}

/*
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
*/
fn valid_height(s: &str) -> bool {
    // this is completely unnecessary and causes multiple loops on the same str, but it's fun
    let t = s
        .chars()
        .rev()
        .take(2)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();

    let hgt: usize = s
        .chars()
        .rev()
        .skip(2)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>()
        .parse()
        .unwrap_or(0);

    match t.as_str() {
        "cm" => 150 <= hgt && hgt <= 193,
        "in" => 59 <= hgt && hgt <= 76,
        _ => false,
    }
    // 160cm
}

#[test]
fn test_valid_height() {
    let cases = [
        ("50in", false),
        ("59in", true),
        ("77in", false),
        ("149cm", false),
        ("150cm", true),
        ("194cm", false),
        ("22parsecs", false),
        ("", false),
    ];

    for case in cases.iter() {
        assert_eq!(case.1, valid_height(case.0), "failed test case {}", case.0);
    }
}

fn valid_hair_color(s: &str) -> bool {
    let mut i = s.chars();
    let start = i.next();
    if start != Some('#') {
        return false;
    }

    let rest: String = i.collect();

    if rest.len() != 6 {
        return false;
    }

    match usize::from_str_radix(&rest[..], 16) {
        Err(_) => false,
        _ => true,
    }
}

#[test]
fn test_valid_hair_color() {
    let cases = [
        ("", false),
        ("#617236", true),
        ("#deadbf", true),
        ("#deadbef", false),
        ("#de34ab", true),
        ("#de34gs", false),
        ("#apples", false),
        ("#    ", false),
    ];

    for case in cases.iter() {
        assert_eq!(
            case.1,
            valid_hair_color(case.0),
            "failed test case {}",
            case.0
        );
    }
}

fn valid_eye_color(s: &str) -> bool {
    match s {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}

fn valid_passport_id(s: &str) -> bool {
    let i: Result<usize, _> = s.parse();
    if let Err(_) = i {
        return false;
    }

    s.len() == 9
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum Field {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
    INVALID,
}

impl Field {
    fn required() -> &'static [Field] {
        static REQUIRED: &'static [Field] = &[
            Field::BYR,
            Field::IYR,
            Field::EYR,
            Field::HGT,
            Field::HCL,
            Field::ECL,
            Field::PID,
        ];
        REQUIRED
    }

    /*
    byr (Birth Year) - four digits; at least 1920 and at most 2002.
    iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
    hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pid (Passport ID) - a nine-digit number, including leading zeroes.
    cid (Country ID) - ignored, missing or not.
    */
    fn validate(&self, s: Option<&str>) -> bool {
        match s {
            Some(s) => match self {
                Field::BYR => valid_year(s, 1920, 2002),
                Field::IYR => valid_year(s, 2010, 2020),
                Field::EYR => valid_year(s, 2020, 2030),
                Field::HGT => valid_height(s),
                Field::HCL => valid_hair_color(s),
                Field::ECL => valid_eye_color(s),
                Field::PID => valid_passport_id(s),
                _ => false,
            },
            _ => false,
        }
    }
}

impl<'a> From<&'a str> for Field {
    fn from(s: &'a str) -> Field {
        match s {
            "byr" => Field::BYR,
            "iyr" => Field::IYR,
            "eyr" => Field::EYR,
            "hgt" => Field::HGT,
            "hcl" => Field::HCL,
            "ecl" => Field::ECL,
            "pid" => Field::PID,
            "cid" => Field::CID,
            _ => Field::INVALID,
        }
    }
}

#[derive(Debug)]
pub struct Passport<'a> {
    data: HashMap<Field, &'a str>,
}

impl<'a> Passport<'a> {
    fn new() -> Passport<'a> {
        Passport {
            data: HashMap::new(),
        }
    }

    pub fn all_fields_present(&self) -> bool {
        Field::required()
            .iter()
            .map(|f| self.data.get(f).unwrap_or(&"").len() > 0)
            .fold(true, |acc, a| acc && a)
    }

    pub fn valid(&self) -> bool {
        Field::required()
            .iter()
            .map(|f| f.validate(self.data.get(f).map(|&v| v)))
            .fold(true, |acc, a| acc && a)
    }
}

impl<'a> TryFrom<&'a str> for Passport<'a> {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut result: Passport = Passport::new();
        for field in s.split_whitespace() {
            let key_value: Vec<&str> = field.splitn(2, ":").collect();

            if key_value.len() != 2 {
                return Err(format!("invalid field: {}", field));
            }

            if let Field::INVALID = key_value[0].into() {
                return Err(format!("invalid field: {}", key_value[0]));
            }

            result.data.insert(key_value[0].into(), key_value[1]);
        }
        Ok(result)
    }
}
