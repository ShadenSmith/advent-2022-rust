//use std::format;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Rock,
    Paper,
    Scissors,
}

pub enum Outcome {
    Lose,
    Draw,
    Win,
}

fn match_value(result: Outcome) -> i32 {
    match result {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

impl State {
    pub fn parse(symbol: &str) -> State {
        return match symbol {
            "A" => State::Rock,
            "B" => State::Paper,
            "C" => State::Scissors,
            "X" => State::Rock,
            "Y" => State::Paper,
            "Z" => State::Scissors,
            &_ => todo!(),
        };
    }

    pub fn value(&self) -> i32 {
        return match self {
            State::Rock => 1,
            State::Paper => 2,
            State::Scissors => 3,
        };
    }

    pub fn score_versus(&self, other: State) -> Outcome {
        match self {
            State::Rock => match other {
                State::Rock => Outcome::Draw,
                State::Paper => Outcome::Lose,
                State::Scissors => Outcome::Win,
            },
            State::Paper => match other {
                State::Rock => Outcome::Win,
                State::Paper => Outcome::Draw,
                State::Scissors => Outcome::Lose,
            },
            State::Scissors => match other {
                State::Rock => Outcome::Lose,
                State::Paper => Outcome::Win,
                State::Scissors => Outcome::Draw,
            },
        }
    }
}

#[derive(Debug)]
pub struct RockPaperScissors {
    p1: State,
    p2: State,
}

impl RockPaperScissors {
    fn parse(player1: &str, player2: &str) -> RockPaperScissors {
        RockPaperScissors {
            p1: State::parse(player1),
            p2: State::parse(player2),
        }
    }

    fn score_p2(&self) -> i32 {
        return self.p2.value() + match_value(self.p2.score_versus(self.p1));
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

        guide.push(RockPaperScissors::parse(&hand[0], &hand[1]));
    }

    return guide;
}

fn compute_desired_hand(hand: &RockPaperScissors) -> State {
    // Part 2: convert X, Y, Z => L, D, W
    let desired: Outcome = match hand.p2 {
        State::Rock => Outcome::Lose,
        State::Paper => Outcome::Draw,
        State::Scissors => Outcome::Win,
    };

    return match hand.p1 {
        State::Rock => match desired {
            Outcome::Lose => State::Scissors,
            Outcome::Draw => State::Rock,
            Outcome::Win => State::Paper,
        },
        State::Paper => match desired {
            Outcome::Lose => State::Rock,
            Outcome::Draw => State::Paper,
            Outcome::Win => State::Scissors,
        },
        State::Scissors => match desired {
            Outcome::Lose => State::Paper,
            Outcome::Draw => State::Scissors,
            Outcome::Win => State::Rock,
        },
    };
}

fn compute_ideal(guide: &[RockPaperScissors]) -> Vec<RockPaperScissors> {
    return guide
        .iter()
        .map(|g| RockPaperScissors {
            p1: g.p1,
            p2: compute_desired_hand(g),
        })
        .collect();
}

pub fn score_strategy_guide(guide: &[RockPaperScissors]) -> i32 {
    return guide.iter().map(|g| g.score_p2()).sum();
}

pub fn score_strategy_guide_pt2(guide: &[RockPaperScissors]) -> i32 {
    let ideal_guide = compute_ideal(guide);
    return score_strategy_guide(&ideal_guide);
}
