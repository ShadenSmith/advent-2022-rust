use day14::sandsim::SandSim;

pub fn main() {
    let mut sim = SandSim::from_path("inputs/main.txt");
    println!("Part 1: {}", sim.sand_capacity());

    let mut sim = SandSim::from_path("inputs/main.txt");
    println!("Part 2: {}", sim.sand_capacity_with_inf_floor());
}
