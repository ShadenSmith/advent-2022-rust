use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::primitive::char;

pub fn find_shared_items(chunks: Vec<&[char]>) -> Vec<char> {
    if chunks.is_empty() {
        return vec![];
    }

    let hashes: Vec<HashSet<char>> = chunks
        .iter()
        .map(|c| HashSet::from_iter(c.to_vec()))
        .collect();

    let mut in_all = hashes[0].clone();

    for hash in hashes.iter().skip(1) {
        in_all = in_all.intersection(hash).cloned().collect()
    }

    in_all.iter().cloned().collect()
}

#[derive(Debug)]
pub struct Rucksack {
    left: Vec<char>,
    right: Vec<char>,
}

impl Rucksack {
    pub fn parse(leftright: &str) -> Rucksack {
        let len = leftright.len();
        if len % 2 != 0 {
            panic!("leftright must be even length");
        }

        let half = len / 2;
        Rucksack {
            left: leftright[0..half].chars().collect(),
            right: leftright[half..len].chars().collect(),
        }
    }

    pub fn left_sack(&self) -> String {
        self.left.iter().collect()
    }

    pub fn right_sack(&self) -> String {
        self.right.iter().collect()
    }

    pub fn find_shared_item(&self) -> char {
        let mut left_set = HashSet::new();
        for item in self.left.iter() {
            left_set.insert(item);
        }

        // Now query right against left
        for item in self.right.iter() {
            if left_set.contains(&item) {
                return *item;
            }
        }

        panic!("No shared item found in rucksack.");
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

pub struct Team {
    pub group_a: Vec<char>,
    pub group_b: Vec<char>,
    pub group_c: Vec<char>,
}

impl Team {
    pub fn parse(a: &str, b: &str, c: &str) -> Team {
        Team {
            group_a: a.chars().collect(),
            group_b: b.chars().collect(),
            group_c: c.chars().collect(),
        }
    }

    pub fn badge(&self) -> char {
        let shared = find_shared_items(vec![&self.group_a, &self.group_b, &self.group_c]);
        if shared.len() != 1 {
            panic!("Found more than 1 item in all groups {:?}", shared);
        }

        shared[0]
    }
}

pub fn parse_teams(path: &str) -> Vec<Team> {
    let reader = BufReader::new(File::open(path).expect("File not found"));

    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    if lines.len() % 3 != 0 {
        panic!("File length must be divisible by 3");
    }

    let mut teams: Vec<Team> = Vec::new();
    for idx in (0..lines.len()).step_by(3) {
        teams.push(Team::parse(&lines[idx], &lines[idx + 1], &lines[idx + 2]));
    }
    teams
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

pub fn overlap_priority(sacks: &[Rucksack]) -> u32 {
    sacks.iter().map(|s| priority(s.find_shared_item())).sum()
}

pub fn badge_priorities(teams: &[Team]) -> u32 {
    teams.iter().map(|t| priority(t.badge())).sum()
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
