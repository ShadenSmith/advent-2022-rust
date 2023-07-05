use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
pub enum Tile {
    Air,
    Rock,
    Sand,
}

pub struct SandSim {
    tiles: HashMap<(i64, i64), Tile>,
    floor: Option<i64>,
}

impl SandSim {
    pub fn new() -> Self {
        SandSim {
            tiles: HashMap::new(),
            floor: None,
        }
    }

    pub fn set_floor(&mut self, depth: i64) {
        self.floor = Some(depth);
    }

    fn get_next_pos(&self, pos: &(i64, i64)) -> Option<(i64, i64)> {
        // First try falling straight down
        let cand = (pos.0, pos.1 + 1);
        if self.get(&cand) == Tile::Air {
            return Some(cand);
        }

        // Diag-left, then diag right
        let cand = (pos.0 - 1, pos.1 + 1);
        if self.get(&cand) == Tile::Air {
            return Some(cand);
        }

        let cand = (pos.0 + 1, pos.1 + 1);
        if self.get(&cand) == Tile::Air {
            return Some(cand);
        }

        None
    }

    pub fn add_sand(&mut self) -> Option<(i64, i64)> {
        let max_depth = self.get_max_depth();

        let mut sand = (500, 0);
        // if already clogged
        if self.get(&sand) != Tile::Air {
            return None;
        }

        loop {
            let next = self.get_next_pos(&sand);

            if next.is_none() {
                self.set(sand, Tile::Sand);
                return Some(sand);
            } else {
                sand = next.unwrap();
            }

            // Falling forever
            if sand.1 > max_depth && self.floor.is_none() {
                return None;
            }
        }
    }

    pub fn sand_capacity(&mut self) -> usize {
        let mut num_sands = 0;

        while self.add_sand().is_some() {
            num_sands += 1;
        }

        num_sands
    }

    pub fn sand_capacity_with_inf_floor(&mut self) -> usize {
        self.set_floor(self.get_max_depth() + 2);

        let mut num_sands = 0;

        while self.add_sand().is_some() {
            num_sands += 1;
        }

        num_sands
    }

    pub fn from_path(path: &str) -> Self {
        let mut sim = SandSim::new();

        let reader = BufReader::new(File::open(path).unwrap());
        for line in reader.lines() {
            let line = line.unwrap();
            let points: Vec<(i64, i64)> = line
                .split(" -> ")
                .map(|pstr| {
                    let xys: Vec<&str> = pstr.split(',').collect();
                    assert_eq!(xys.len(), 2);
                    (xys[0].parse().unwrap(), xys[1].parse().unwrap())
                })
                .collect();

            for (idx, p2) in points.iter().cloned().enumerate().skip(1) {
                let mut p1 = points[idx - 1];

                sim.set(p1, Tile::Rock);

                while p1 != p2 {
                    let clamp = |v| min(max(v, -1), 1);
                    p1.0 += clamp(p2.0 - p1.0);
                    p1.1 += clamp(p2.1 - p1.1);
                    sim.set(p1, Tile::Rock);
                }
            }
        }

        sim
    }

    pub fn get(&self, pos: &(i64, i64)) -> Tile {
        // Check for infinite floor
        if let Some(floor) = self.floor {
            if pos.1 == floor {
                return Tile::Rock;
            }
        }

        self.tiles.get(pos).unwrap_or(&Tile::Air).clone()
    }

    fn set(&mut self, pos: (i64, i64), v: Tile) {
        self.tiles.insert(pos, v);
    }

    pub fn get_max_depth(&self) -> i64 {
        self.floor
            .unwrap_or(*self.tiles.keys().map(|(_, y)| y).max().unwrap())
    }

    pub fn draw(&self) {
        // Get the min and max column for width
        let min_col = self.tiles.keys().map(|(x, _)| x).min().unwrap().clone();
        let max_col = self.tiles.keys().map(|(x, _)| x).max().unwrap().clone();

        let max_row = self.get_max_depth();
        for row in 0..max_row {
            for col in min_col..max_col {
                let t = self.get(&(col, row));
                match t {
                    Tile::Air => print!("."),
                    Tile::Sand => print!("o"),
                    Tile::Rock => print!("#"),
                }
            }
            println!();
        }
    }
}

impl Default for SandSim {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sim() {
        let sim = SandSim::from_path("inputs/test.txt");

        assert_eq!(sim.get_max_depth(), 9);

        assert_eq!(sim.get(&(498, 4)), Tile::Rock);
        assert_eq!(sim.get(&(498, 5)), Tile::Rock);
        assert_eq!(sim.get(&(498, 6)), Tile::Rock);

        assert_eq!(sim.get(&(498, 6)), Tile::Rock);
        assert_eq!(sim.get(&(497, 6)), Tile::Rock);
        assert_eq!(sim.get(&(496, 6)), Tile::Rock);

        assert_eq!(sim.get(&(503, 4)), Tile::Rock);
        assert_eq!(sim.get(&(502, 4)), Tile::Rock);
        assert_eq!(sim.get(&(502, 9)), Tile::Rock);
        assert_eq!(sim.get(&(501, 9)), Tile::Rock);
        assert_eq!(sim.get(&(495, 9)), Tile::Rock);
        assert_eq!(sim.get(&(494, 9)), Tile::Rock);

        assert_eq!(sim.get(&(500, 2)), Tile::Air);
        assert_eq!(sim.get(&(502, 2)), Tile::Air);
        assert_eq!(sim.get(&(492, 10)), Tile::Air);
    }
}
