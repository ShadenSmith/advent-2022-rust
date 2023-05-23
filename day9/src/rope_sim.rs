use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Move {
    Right,
    Left,
    Up,
    Down,
}

impl Move {
    pub fn parse(s: &str) -> Self {
        match s {
            "R" => Move::Right,
            "L" => Move::Left,
            "U" => Move::Up,
            "D" => Move::Down,
            _ => panic!("Invalid move char"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    pub fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn step(&self, direction: &Move) -> Self {
        match direction {
            Move::Left => Self {
                y: self.y,
                x: self.x - 1,
            },
            Move::Right => Self {
                y: self.y,
                x: self.x + 1,
            },
            Move::Up => Self {
                y: self.y + 1,
                x: self.x,
            },
            Move::Down => Self {
                y: self.y - 1,
                x: self.x,
            },
        }
    }

    pub fn translate(&self, delta_x: i64, delta_y: i64) -> Self {
        Self::new(self.x + delta_x, self.y + delta_y)
    }

    fn grad(&self, other: &Self) -> Self {
        Self::new(other.x - self.x, other.y - self.y)
    }

    pub fn is_touching(&self, other: &Self) -> bool {
        let g = self.grad(&other);

        // abs(x) < 2 and abs(y) < 2
        (-1..2).contains(&g.x) && (-1..2).contains(&g.y)
    }

    fn follow(&self, &head: &Self) -> Self {
        if self.is_touching(&head) {
            return self.clone();
        }

        let clamp = |v| {
            if v < -1 {
                -1
            } else if v > 1 {
                1
            } else {
                v
            }
        };
        let grad = (clamp(head.x - self.x), clamp(head.y - self.y));

        return self.translate(grad.0, grad.1);
    }
}

pub fn parse_sim(path: &str) -> Vec<Move> {
    let reader = BufReader::new(File::open(path).expect("File not found"));

    let mut moves = vec![];

    for line in reader.lines() {
        // Ex: D 4
        let line = line.unwrap();
        let line: Vec<&str> = line.split(" ").collect();
        let dir = Move::parse(line[0]);
        let count = line[1].parse::<u64>().unwrap();

        for _ in 0..count {
            moves.push(dir);
        }
    }

    moves
}

pub fn count_tail_positions(sim_cmds: &Vec<Move>) -> u64 {
    let mut head = Coord::origin();
    let mut tail = Coord::origin();

    let mut tail_seen = HashSet::new();
    tail_seen.insert(tail.clone());

    for cmd in sim_cmds.iter() {
        head = head.step(cmd);
        tail = tail.follow(&head);
        tail_seen.insert(tail.clone());
    }

    tail_seen.len().try_into().unwrap()
}

pub fn count_tail_positions_in_chain(sim_cmds: &Vec<Move>, chain_size: usize) -> u64 {
    let mut chain = Vec::new();
    for _ in 0..chain_size {
        chain.push(Coord::origin());
    }

    let head_idx = 0;
    let tail_idx = chain.len() - 1;

    let mut tail_seen = HashSet::new();
    tail_seen.insert(chain[tail_idx].clone());

    for cmd in sim_cmds.iter() {
        chain[head_idx] = chain[head_idx].step(cmd);
        for idx in 1..chain.len() {
            chain[idx] = chain[idx].follow(&chain[idx - 1]);
        }
        tail_seen.insert(chain[tail_idx].clone());
    }

    tail_seen.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_move() {
        assert_eq!(Move::parse("R"), Move::Right);
        assert_eq!(Move::parse("L"), Move::Left);
        assert_eq!(Move::parse("U"), Move::Up);
        assert_eq!(Move::parse("D"), Move::Down);
    }

    #[test]
    fn test_coord_move() {
        assert_eq!(Coord::origin().step(&Move::Left), Coord { y: 0, x: -1 });
        assert_eq!(Coord::origin().step(&Move::Right), Coord { y: 0, x: 1 });
        assert_eq!(Coord::origin().step(&Move::Up), Coord { y: 1, x: 0 });
        assert_eq!(Coord::origin().step(&Move::Down), Coord { y: -1, x: 0 });
    }

    #[test]
    fn test_parse_sim() {
        let sim = parse_sim("input_test.txt");
        assert_eq!(sim.len(), 24);
    }
}
