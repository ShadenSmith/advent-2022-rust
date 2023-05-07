use day5::*;

pub fn main() {
    let (mut ship, steps) = parse_crate_file("day5/input.txt");

    ship.execute(&steps);

    println!("{}", ship.top_stacks());
}
