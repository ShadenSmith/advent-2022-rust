use day12::rescue_map;

pub fn main() {
    let map = rescue_map::RescueMap::from_path("day12/inputs/main.txt");
    println!("Part 1: {}", map.shortest_path());
}
