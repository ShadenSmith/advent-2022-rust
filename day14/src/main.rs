use day14::sandsim::SandSim;

pub fn main() {
    let mut sim = SandSim::from_path("inputs/main.txt");
    println!("Part 1: {}", sim.sand_capacity());
}
