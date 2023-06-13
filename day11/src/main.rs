use day11::monkey::MonkeySystem;
use day11::worry::Worry;

pub fn main() {
    let mut ms = MonkeySystem::from_path("day11/inputs/part1.txt");
    println!("Part 1: {}", ms.monkey_business(20, Worry(3)));

    let mut ms = MonkeySystem::from_path("day11/inputs/part1.txt");
    println!("Part 2: {}", ms.monkey_business(10_000, Worry(1)));
}
