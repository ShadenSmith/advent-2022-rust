

//use std::format;
use std::fs::File;
use std::io::{BufRead,BufReader};


#[derive(Debug)]
pub enum RPS {
    ROCK,
    PAPER,
    SCISSORS,
}

impl RPS {
    pub fn new(symbol: &str) -> RPS {
        return match symbol {
            "A" => RPS::ROCK,
            "B" => RPS::PAPER,
            "C" => RPS::SCISSORS,
            "X" => RPS::ROCK,
            "Y" => RPS::PAPER,
            "Z" => RPS::SCISSORS,
            &_ => todo!(),
        };
    }

    pub fn value(self) -> i32 {
        return match self {
            RPS::ROCK => 1,
            RPS::PAPER => 2,
            RPS::SCISSORS => 3,
        }
    }
}

#[derive(Debug)]
pub struct RockPaperScissors {
    p1: RPS,
    p2: RPS,
}

impl RockPaperScissors {
    fn new(player1: &str, player2: &str) -> RockPaperScissors {
        RockPaperScissors {
            p1: RPS::new(player1),
            p2: RPS::new(player2),
        }
    }

    fn score_p2(self) -> i32 {
        return self.p2.value();
    }
}

pub fn parse_strategy_guide(path: &str) -> Vec<RockPaperScissors> {

    let mut guide: Vec<RockPaperScissors> = vec![];
    let reader = BufReader::new(File::open(path).unwrap());
    for line in reader.lines() {
        let line = line.unwrap();

        let mut hand: Vec<String> = vec![];
        for inp in line.split_whitespace() {
            hand.push(String::from(inp));
        }
        assert_eq!(hand.len(), 2);

        guide.push(RockPaperScissors::new(&hand[0], &hand[1]));
    }

    return guide
}

pub fn score_strategy_guide(guide: &[RockPaperScissors]) -> i32 {
    -1
}