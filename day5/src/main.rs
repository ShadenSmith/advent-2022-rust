use day5::*;

fn part1() {
    let (mut ship, steps) = parse_crate_file("day5/input.txt");
    ship.execute(&steps);
    println!("{}", ship.top_stacks());
}

fn part2() {
    let (mut ship, steps) = parse_crate_file("day5/input.txt");
    ship.execute_pt2(&steps);
    println!("{}", ship.top_stacks());
}

pub fn main() {
    part1();
    part2();
}
