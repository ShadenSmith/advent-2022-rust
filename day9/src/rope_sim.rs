#[derive(PartialEq, Debug)]
enum Move {
    Right(u64),
    Left(u64),
    Up(u64),
    Down(u64),
}

impl Move {
    pub fn parse(s: &str) -> Self {
        Move::Right(3)
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
        assert_eq!(Move::parse("R 3"), Move::Right(3));
        assert_eq!(Move::parse("L 2"), Move::Left(3));
        assert_eq!(Move::parse("U 8"), Move::Up(8));
        assert_eq!(Move::parse("D 1"), Move::Down(1));
    }
}
