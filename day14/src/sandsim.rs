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
}

impl SandSim {
    pub fn new() -> Self {
        SandSim {
            tiles: HashMap::new(),
        }
    }

    fn get_next_pos(&self, pos: &(i64, i64)) -> Option<(i64, i64)> {
        let mut cand = pos.clone();

        // First try falling straight down
        cand = (pos.0, pos.1 + 1);
        if self.get(&cand) == Tile::Air {
            return Some(cand);
        }

        // Diag-left, then diag right
        cand = (pos.0 - 1, pos.1 + 1);
        if self.get(&cand) == Tile::Air {
            return Some(cand);
        }
        cand = (pos.0 + 1, pos.1 + 1);
        if self.get(&cand) == Tile::Air {
            return Some(cand);
        }

        None
    }

    pub fn add_sand(&mut self) -> Option<(i64, i64)> {
        let max_depth = self.get_max_depth();

        let mut sand = (500, 0);
        loop {
            let next = self.get_next_pos(&sand);

            if next.is_some() {
                sand = next.unwrap();
            } else {
                self.set(sand.clone(), Tile::Sand);
                return Some(sand);
            }

            // Falling forever
            if sand.1 > max_depth {
                return None;
            }
        }
    }

    pub fn sand_capacity(&mut self) -> usize {
        let mut num_sands = 0;

        while let Some(_) = self.add_sand() {
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
                    let xys: Vec<&str> = pstr.split(",").collect();
                    if xys.len() != 2 {
                        panic!("bad coord");
                    }
                    return (xys[0].parse().unwrap(), xys[1].parse().unwrap());
                })
                .collect();

            for (idx, p2) in points.iter().cloned().enumerate().skip(1) {
                let mut p1 = points[idx - 1];

                sim.set(p1.clone(), Tile::Rock);

                while p1 != p2 {
                    let clamp = |v| min(max(v, -1), 1);
                    p1.0 += clamp(p2.0 - p1.0);
                    p1.1 += clamp(p2.1 - p1.1);
                    sim.set(p1.clone(), Tile::Rock);
                }
            }
        }

        sim
    }

    pub fn get(&self, pos: &(i64, i64)) -> Tile {
        self.tiles.get(pos).unwrap_or(&Tile::Air).clone()
    }

    fn set(&mut self, pos: (i64, i64), v: Tile) {
        self.tiles.insert(pos, v);
    }

    pub fn get_max_depth(&self) -> i64 {
        self.tiles.keys().map(|(_, y)| y).max().unwrap().clone()
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
