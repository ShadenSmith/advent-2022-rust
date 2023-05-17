
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct HeightMap {
    pub num_rows: u64,
    pub num_cols: u64,
    pub heights: Vec<Vec<u32>>,
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

    pub fn get(&self, row: usize, col: usize) -> u32 {
        self.heights[row][col]
    }

    pub fn count_visible_trees(&self) -> u64 {
        // -4 to avoid double counting corners
        let edge_count = (self.num_rows * 2) + (self.num_cols*2) - 4;

        // Now go over rows and columns 
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_path() {
        let hmap = HeightMap::from_path("input_test.txt");

        assert_eq!(hmap.num_rows, 5);
        assert_eq!(hmap.num_cols, 5);
        assert_eq!(hmap.get(0,0), 3);
        assert_eq!(hmap.heights[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(hmap.heights[4], vec![3, 5, 3, 9, 0]);
    }
}
