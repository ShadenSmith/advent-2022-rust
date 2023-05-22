use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct HeightMap {
    pub num_rows: usize,
    pub num_cols: usize,
    pub heights: Vec<Vec<u32>>,
}

impl HeightMap {
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
            num_rows: parsed_heights.len(),
            num_cols: parsed_heights[0].len(),
            heights: parsed_heights,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> u32 {
        self.heights[row][col]
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || row == self.heights.len() - 1 {
            return true;
        }

        if col == 0 || col == self.heights[row].len() - 1 {
            return true;
        }

        let my_height = self.get(row, col);

        // Check row
        let left = &self.heights[row][0..col];
        if left.iter().all(|h| h < &my_height) {
            return true;
        }

        let right = &self.heights[row][col + 1..self.heights[row].len()];
        if right.iter().all(|h| h < &my_height) {
            return true;
        }

        let top = (0..row).map(|r| self.get(r, col)).all(|h| h < my_height);
        if top {
            return true;
        }

        let bot = (row + 1..self.heights.len())
            .map(|r| self.get(r, col))
            .all(|h| h < my_height);
        if bot {
            return true;
        }

        false
    }

    pub fn count_visible_trees(&self) -> u32 {
        // with NxN grid, this is O(N^3)
        // TODO: O(N^2) algo via linear passes in each direction
        let mut marker: Vec<bool> = vec![];

        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                marker.push(self.is_visible(row, col));
            }
        }

        let num = marker.into_iter().filter(|v| *v).count();
        num.try_into().unwrap()
    }

    fn scenic_score(&self, row: usize, col: usize) -> u32 {
        let left = (0..col).map(|c| self.get(row, c)).rev();
        let right = (col + 1..self.heights[row].len()).map(|c| self.get(row, c));
        let top = (0..row).map(|r| self.get(r, col)).rev();
        let bot = (row + 1..self.heights.len()).map(|r| self.get(r, col));

        let my_height = self.get(row, col);

        let count_fn = |h_iter: Vec<u32>| {
            let mut num_vis = 0;
            for h in h_iter.into_iter() {
                num_vis += 1;
                if h >= my_height {
                    return num_vis;
                }
            }
            num_vis
        };

        let scenic_counts = vec![
            count_fn(left.collect_vec()),
            count_fn(right.collect_vec()),
            count_fn(top.collect_vec()),
            count_fn(bot.collect_vec()),
        ];

        scenic_counts.into_iter().product()
    }

    pub fn max_scenic(&self) -> u32 {
        let mut scores = vec![];
        for row in 0..self.heights.len() {
            for col in 0..self.heights[row].len() {
                scores.push(self.scenic_score(row, col));
            }
        }

        scores.into_iter().max().unwrap()
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
        assert_eq!(hmap.get(0, 0), 3);
        assert_eq!(hmap.heights[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(hmap.heights[4], vec![3, 5, 3, 9, 0]);
    }
}
