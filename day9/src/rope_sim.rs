#[derive(PartialEq, Debug)]
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
        RopeSim { cmds: vec![] }
    }

    pub fn count_tail_positions(&self) -> u64 {
        0
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_move() {
        use super::Move;
        assert_eq!(Move::parse("R"), Move::Right);
        assert_eq!(Move::parse("L"), Move::Left);
        assert_eq!(Move::parse("U"), Move::Up);
        assert_eq!(Move::parse("D"), Move::Down);
    }
}
