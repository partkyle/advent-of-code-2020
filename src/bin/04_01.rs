/*
--- Day 4: Passport Processing ---
You arrive at the airport only to realize that you grabbed your North Pole Credentials instead of your passport. While these documents are extremely similar, North Pole Credentials aren't issued by a country and therefore aren't actually valid documentation for travel in most of the world.

It seems like you're not the only one having problems, though; a very long line has formed for the automatic passport scanners, and the delay could upset your travel itinerary.

Due to some questionable network security, you realize you might be able to solve both of these problems at the same time.

The automatic passport scanners are slow because they're having trouble detecting which passports have all required fields. The expected fields are as follows:

byr (Birth Year)
iyr (Issue Year)
eyr (Expiration Year)
hgt (Height)
hcl (Hair Color)
ecl (Eye Color)
pid (Passport ID)
cid (Country ID)
Passport data is validated in batch files (your puzzle input). Each passport is represented as a sequence of key:value pairs separated by spaces or newlines. Passports are separated by blank lines.

Here is an example batch file containing four passports:

ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
The first passport is valid - all eight fields are present. The second passport is invalid - it is missing hgt (the Height field).

The third passport is interesting; the only missing field is cid, so it looks like data from North Pole Credentials, not a passport at all! Surely, nobody would mind if you made the system temporarily ignore missing cid fields. Treat this "passport" as valid.

The fourth passport is missing two fields, cid and byr. Missing cid is fine, but missing any other field is not, so this passport is invalid.

According to the above rules, your improved system would report 2 valid passports.

Count the number of valid passports - those that have all required fields. Treat cid as optional. In your batch file, how many passports are valid?
*/
use std::collections::HashMap;
use std::convert::{From, Into, TryFrom, TryInto};
use std::fs;

const FILENAME: &str = "files/04/input.txt";

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
struct Passport<'a> {
    data: HashMap<Field, &'a str>,
}

impl<'a> Passport<'a> {
    fn new() -> Passport<'a> {
        Passport {
            data: HashMap::new(),
        }
    }

    fn valid(&self) -> bool {
        Field::required()
            .iter()
            .map(|f| self.data.get(f).unwrap_or(&"").len() > 0)
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

fn main() {
    let input = fs::read_to_string(FILENAME).expect("couldn't open input file");

    let passports: Result<Vec<Passport>, _> = input.split("\n\n").map(|e| e.try_into()).collect();

    let answer = passports.expect("could not load passports").iter().filter(|p| p.valid()).count();

    println!("answer: {}", answer);
}
