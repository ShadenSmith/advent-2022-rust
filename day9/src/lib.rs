pub mod rope_sim;

#[cfg(test)]
mod tests {
    use crate::rope_sim::RopeSim;

    #[test]
    fn test_part1() {
        let sim = RopeSim::from_path("input_test.txt");
        assert_eq!(sim.count_tail_positions(), 13);
    }
}
