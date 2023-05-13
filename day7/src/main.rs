mod babyfs;

use babyfs::fs::FileSystem;

pub fn main() {
    let fs = FileSystem::from_path("day7/input.txt").unwrap();

    println!("Part 1: {}", fs.part1());
}
