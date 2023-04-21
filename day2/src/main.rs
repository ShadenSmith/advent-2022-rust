

use day2::*;

fn main() {
    let guide = parse_strategy_guide("input.txt");
    println!("{:?}", guide);

    println!("{:?}", score_strategy_guide(&guide));
}