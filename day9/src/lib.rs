pub mod rope_sim;

#[cfg(test)]
mod tests {
    use crate::rope_sim::*;

    #[test]
    fn test_part1() {
        let sim_cmds = parse_sim("input_test.txt");
        assert_eq!(count_tail_positions(&sim_cmds), 13);
    }

    #[test]
    fn test_part2a() {
        let sim_cmds = parse_sim("input_test.txt");
        assert_eq!(count_tail_positions_in_chain(&sim_cmds, 10), 1);
    }

    #[test]
    fn test_part2b() {
        let sim_cmds = parse_sim("input_test2.txt");
        assert_eq!(count_tail_positions_in_chain(&sim_cmds, 10), 36);
    }
}
