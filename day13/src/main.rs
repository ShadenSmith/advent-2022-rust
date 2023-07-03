use day13::packets;

pub fn main() {
    let part1 = packets::num_misordered_packets("inputs/main.txt");
    let part2 = packets::part2("inputs/main.txt");

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
