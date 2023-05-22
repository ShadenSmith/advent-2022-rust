
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
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

pub struct RopeSim {
    cmds: Vec<Move>,
}

impl RopeSim {
    pub fn from_path(path: &str) -> Self {
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

        RopeSim { cmds: moves }
    }

    pub fn len(&self) -> usize {
        self.cmds.len()
    }

    pub fn count_tail_positions(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Move;
    use super::RopeSim;


    #[test]
    fn test_parse_move() {
        assert_eq!(Move::parse("R"), Move::Right);
        assert_eq!(Move::parse("L"), Move::Left);
        assert_eq!(Move::parse("U"), Move::Up);
        assert_eq!(Move::parse("D"), Move::Down);
    }

    #[test]
    fn test_parse_sim() {
        let sim = RopeSim::from_path("input_test.txt");
        assert_eq!(sim.len(), 24);
    }
}
