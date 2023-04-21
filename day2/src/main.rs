

use day2::*;

fn main() {
    let guide = parse_strategy_guide("input.txt");
    println!("Part-1: {:?}", score_strategy_guide(&guide));
    println!("Part-2: {:?}", score_strategy_guide_pt2(&guide));
}