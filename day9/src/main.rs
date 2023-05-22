mod rope_sim;

use rope_sim::RopeSim;

pub fn main() {
    let sim = RopeSim::from_path("input_test.txt");
    println!("Part 1: {}", sim.count_tail_positions());
    println!("Part 2: {}", 0);
}
