mod rope_sim;

pub fn main() {
    let sim = rope_sim::parse_sim("day9/input.txt");
    println!("Part 1: {}", rope_sim::count_tail_positions(&sim));
    println!(
        "Part 2: {}",
        rope_sim::count_tail_positions_in_chain(&sim, 10)
    );
}
