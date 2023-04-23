use day4::*;

fn main() {
    let ranges = parse_range_file("day4/input.txt");
    println!("Part 1: {}", count_contained_ranges(&ranges));
    println!("Part 2: {}", count_overlapping_ranges(&ranges));
}
