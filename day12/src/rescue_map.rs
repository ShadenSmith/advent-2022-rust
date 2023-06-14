use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq)]
pub enum MapCell {
    Start,
    Terrain(char),
    End,
}

impl MapCell {
    pub fn parse(c: char) -> Self {
        match c {
            'S' => MapCell::Start,
            'E' => MapCell::End,
            _ => MapCell::Terrain(c),
        }
    }

    pub fn is_traversable(src: &MapCell, dst: &MapCell) -> bool {
        let cmp_terrain = |src: &char, dst: &char| -> bool {
            // you can go across or down
            if src >= dst {
                true
            } else {
                let left = src.clone() as i32;
                let right = dst.clone() as i32;
                right - left <= 1
            }
        };

        match (src, dst) {
            // Map Start to 'a'
            (MapCell::Start, MapCell::Terrain('a')) => true,
            (MapCell::Start, MapCell::Terrain('b')) => true,
            (MapCell::Start, _) => false,
            (_, MapCell::Start) => false, // no loops

            // Map End to 'z'
            (MapCell::Terrain('y'), MapCell::End) => true,
            (MapCell::Terrain('z'), MapCell::End) => true,
            (_, MapCell::End) => false, // nothing but y and z
            (MapCell::End, _) => false, // sink node

            (MapCell::Terrain(left), MapCell::Terrain(right)) => cmp_terrain(left, right),
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            MapCell::Start => 'a' as i32,
            MapCell::End => 'z' as i32,
            MapCell::Terrain(c) => *c as i32,
        }
    }

}

pub struct RescueMap {
    num_rows: usize,
    num_cols: usize,

    pub heights: Vec<Vec<MapCell>>,
}

impl RescueMap {
    pub fn from_path(path: &str) -> Self {
        let reader = BufReader::new(File::open(path).expect("File not found"));

        let mut parsed_heights = vec![];

        let mut start = (0, 0);
        let mut end = (0, 0);

        for line in reader.lines() {
            // Parse the line into a sequence of single-digit integers
            let line_heights: Vec<MapCell> = line.unwrap().chars().map(MapCell::parse).collect();
            parsed_heights.push(line_heights);
        }

        Self {
            num_rows: parsed_heights.len(),
            num_cols: parsed_heights[0].len(),
            heights: parsed_heights,
        }
    }

    pub fn get(&self, coord: (&usize, &usize)) -> MapCell {
        self.heights[*coord.0][*coord.1].clone()
    }

    fn find_start(&self) -> (usize, usize) { 
        for row in 0..self.num_rows {
            if let Some(col) = self.heights[row].iter().position(|p| *p == MapCell::Start) {
                return (row, col)
            }
        }

        unreachable!("Start not found!")
    }

    pub fn shortest_path(&self) -> usize {
        // Find start

        let start = self.find_start();

        println!("start: {:?}", start);

        0
    }
}
