use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub lo: i32,
    pub hi: i32,
}

impl Range {
    pub fn parse(lo_hi: &str) -> Range {
        let parsed: Vec<i32> = lo_hi
            .split("-")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if parsed.len() != 2 {
            panic!("Expected a string of format <int>-<int>");
        }
        Range {
            lo: parsed[0],
            hi: parsed[1],
        }
    }

    pub fn is_contained_in(&self, other: &Range) -> bool {
        (self.lo >= other.lo) && (self.hi <= other.hi)
    }

    pub fn is_overlapping(&self, other: &Range) -> bool {
        ((self.lo >= other.lo) && (self.lo <= other.hi))
            || ((self.hi >= other.lo) && (self.hi <= other.hi))
    }
}

pub fn count_contained_ranges(ranges: &[(Range, Range)]) -> usize {
    ranges
        .iter()
        .filter(|p| p.0.is_contained_in(&p.1) || p.1.is_contained_in(&p.0))
        .count()
}

pub fn count_overlapping_ranges(ranges: &[(Range, Range)]) -> usize {
    ranges
        .iter()
        .filter(|p| p.0.is_overlapping(&p.1) || p.1.is_overlapping(&p.0))
        .count()
}

pub fn parse_range_file(path: &str) -> Vec<(Range, Range)> {
    let mut ranges: Vec<(Range, Range)> = vec![];

    let reader = BufReader::new(File::open(path).expect("File not found"));
    for line in reader.lines() {
        let parsed_ranges: Vec<Range> = line.unwrap().split(",").map(Range::parse).collect();
        ranges.push((parsed_ranges[0], parsed_ranges[1]));
    }

    ranges
}
