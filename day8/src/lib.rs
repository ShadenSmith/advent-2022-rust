use std::fs::File;
use std::io::{BufRead, BufReader};

struct HeightMap {
    num_rows: u64,
    num_cols: u64,
    heights: Vec<Vec<u32>>,
}

impl HeightMap {
    pub fn new() -> Self {
        HeightMap {
            num_rows: 0,
            num_cols: 0,
            heights: vec![vec![]],
        }
    }

    pub fn from_path(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect("File not found"));

        let mut parsed_heights = vec![];

        for line in reader.lines() {
            // Parse the line into a sequence of single-digit integers
            let line_heights: Vec<u32> = line
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            parsed_heights.push(line_heights);
        }

        Self {
            num_rows: parsed_heights.len().try_into().unwrap(),
            num_cols: parsed_heights[0].len().try_into().unwrap(),
            heights: parsed_heights,
        }
    }

    pub fn count_visible_trees(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let hmap = HeightMap::from_path("input_test.txt");
        assert_eq!(hmap.count_visible_trees(), 21);
    }
}
