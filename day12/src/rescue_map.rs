use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp::Reverse;

use std::collections::{BinaryHeap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    row: usize,
    col: usize,
}

impl Coord {
    pub fn new(row: usize, col: usize) -> Self {
        Coord { row, col }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SearchNode {
    cost: usize,
    state: Coord,
}

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

    pub fn shortest_path(&self) -> usize {
        // Find start
        let start = self.find_start();

        let mut explored = HashSet::new();

        let mut pq = BinaryHeap::new();
        pq.push(Reverse(SearchNode {
            state: start.clone(),
            cost: 0,
        }));

        let mut num_seen = 0;
        while let Some(Reverse(node)) = pq.pop() {
            if self.get(&node.state) == MapCell::End {
                return node.cost;
            }

            for nbr in self.get_reachable_from(&node.state) {
                if !explored.contains(&nbr) {
                    pq.push(Reverse(SearchNode {
                        cost: node.cost + 1,
                        state: nbr.clone(),
                    }));

                    explored.insert(nbr.clone());
                }
            }
        }

        panic!("No path to end found.");
    }

    pub fn get(&self, coord: &Coord) -> MapCell {
        self.heights[coord.row][coord.col].clone()
    }

    fn find_start(&self) -> Coord {
        for row in 0..self.num_rows {
            if let Some(col) = self.heights[row].iter().position(|p| *p == MapCell::Start) {
                return Coord::new(row, col);
            }
        }

        unreachable!("Start not found!")
    }

    fn get_reachable_from(&self, pos: &Coord) -> Vec<Coord> {
        let mut nbrs = Vec::new();

        let curr_cell = self.get(&pos);

        let mut maybe_traverse_nbr = |next_coord: Coord| {
            let next_cell = self.get(&next_coord);
            if MapCell::is_traversable(&curr_cell, &next_cell) {
                nbrs.push(next_coord);
            }
        };

        if pos.row > 0 {
            let next_coord = Coord::new(pos.row - 1, pos.col);
            maybe_traverse_nbr(next_coord);
        }
        if pos.row < self.num_rows - 1 {
            let next_coord = Coord::new(pos.row + 1, pos.col);
            maybe_traverse_nbr(next_coord);
        }
        if pos.col > 0 {
            let next_coord = Coord::new(pos.row, pos.col - 1);
            maybe_traverse_nbr(next_coord);
        }
        if pos.col < self.num_cols - 1 {
            let next_coord = Coord::new(pos.row, pos.col + 1);
            maybe_traverse_nbr(next_coord);
        }

        nbrs
    }
}
