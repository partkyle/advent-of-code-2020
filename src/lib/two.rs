use std::str::FromStr;

#[derive(Debug)]
pub struct Item {
    pub min: u8,
    pub max: u8,
    pub letter: String,
    pub password: String,
}

impl FromStr for Item {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_split: Vec<&str> = s.split(":").collect();

        if colon_split.len() != 2 {
            return Err("invalid parse. Wrong number of elements past `:`".to_string());
        }

        let range_letter_split: Vec<&str> = colon_split[0].split(" ").collect();

        if range_letter_split.len() != 2 {
            return Err("invalid parse. Wrong number of elements past `-`".to_string());
        }

        let min_max: Result<Vec<u8>, _> = range_letter_split[0]
            .split("-")
            .map(|e| e.parse())
            .collect();

        let (min, max) = match min_max {
            Err(err) => return Err(format!("could not get min and max: {}", err)),
            Ok(min_max) => (min_max[0], min_max[1]),
        };

        Ok(Item {
            min: min,
            max: max,
            letter: range_letter_split[1].trim().to_string(),
            password: colon_split[1].trim().to_string(),
        })
    }
}
