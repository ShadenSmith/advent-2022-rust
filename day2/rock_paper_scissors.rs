
use std::fs;
use std::io::{BufRead,BufReader};


enum RPS {
    ROCK,
    PAPER,
    SCISSORS
}

impl RPS {
    fn new(symbol: &str) -> RPS {
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
}

fn main() {
    let fname = "input.txt";
    let reader = BufReader::new(fs::File::open(fname).unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        //let hand: Vec[RPS]::new();
        for c in line.split_whitespace() {
            
        }
    }
}