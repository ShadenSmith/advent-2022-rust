mod heightmap;
use heightmap::HeightMap;

pub fn main() {
    let hmap = HeightMap::from_path("day8/input.txt");
    println!("Part 1: {}", hmap.count_visible_trees());
}
