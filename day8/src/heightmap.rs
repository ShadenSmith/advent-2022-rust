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

    pub fn count_visible_trees(&self) -> usize {
        // -4 to avoid double counting corners
        let edge_count = (self.num_rows * 2) + (self.num_cols * 2) - 2;

        // Initialize each tree as visible
        let mut marker: Vec<Vec<bool>> = vec![];
        for _ in 0..self.num_rows {
            let mut rowmarker = vec![];
            rowmarker.resize(self.num_cols, false);
            marker.push(rowmarker);
        }

        // Now go over interior trees in each direction and mark when a tree becomes invisible
        // left -> right
        for row in 1..marker.len() - 1 {
            let mut max_seen = self.get(row, 0);
            for col in 1..self.num_cols - 1 {
                if self.get(row, col) > max_seen {
                    marker[row][col] = true;
                    max_seen = self.get(row, col);
                }
            }
        }

        // right -> left
        for row in 1..marker.len() - 1 {
            let mut max_seen = self.get(row, self.num_cols - 1);
            for col in self.num_cols - 1..0 {
                if self.get(row, col) > max_seen {
                    marker[row][col] = true;
                    max_seen = self.get(row, col);
                }
            }
        }

        // top -> bottom
        for col in 1..self.num_cols - 1 {
            let mut max_seen = self.get(0, col);
            for row in 1..marker.len() - 1 {
                if self.get(row, col) > max_seen {
                    marker[row][col] = true;
                    max_seen = self.get(row, col);
                }
            }
        }

        // bottom -> top
        for col in 1..self.num_cols - 1 {
            let mut max_seen = self.get(self.num_rows - 1, col);
            for row in marker.len() - 1..0 {
                if self.get(row, col) > max_seen {
                    marker[row][col] = true;
                    max_seen = self.get(row, col);
                }
            }
        }

        let interior: usize = marker
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum();
        edge_count + interior
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
