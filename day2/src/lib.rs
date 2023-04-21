pub fn add(left: usize, right: usize) -> usize {
    left + right
}

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
