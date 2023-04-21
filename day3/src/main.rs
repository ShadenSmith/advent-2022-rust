use day3::*;

fn main() {
    let sacks = parse_rucksacks("day3/input.txt");
    println!("Part 1: {}", overlap_priority(&sacks));

    let teams = parse_teams("day3/input.txt");
    println!("Part 2: {}", badge_priorities(&teams));
}
