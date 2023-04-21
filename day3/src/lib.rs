use std::fs::File;
use std::io::{BufRead, BufReader};
use std::primitive::char;

#[derive(Debug)]
pub struct Rucksack {
    pub left: String,
    pub right: String,
}

impl Rucksack {
    pub fn parse(leftright: &str) -> Rucksack {
        let len = leftright.len();
        if len % 2 != 0 {
            panic!("leftright must be even length");
        }

        let half = len / 2;
        Rucksack {
            left: leftright[0..half].to_string(),
            right: leftright[half..len].to_string(),
        }
    }
}

pub fn parse_rucksacks(path: &str) -> Vec<Rucksack> {
    let reader = BufReader::new(File::open(path).expect("File not found"));

    let sacks: Vec<Rucksack> = reader
        .lines()
        .map(|s| Rucksack::parse(&s.unwrap()))
        .collect();
    sacks
}

fn priority(token: char) -> u32 {
    if !token.is_ascii_alphabetic() {
        panic!("char must be [a-zA-Z]");
    }

    let (base_char, offset): (char, u32) = if token.is_lowercase() {
        ('a', 1)
    } else {
        ('A', 27)
    };

    offset + u32::from(token) - u32::from(base_char)
}

#[cfg(test)]
mod tests_day3 {
    use super::*;

    #[test]
    fn test_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('b'), 2);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }
}
