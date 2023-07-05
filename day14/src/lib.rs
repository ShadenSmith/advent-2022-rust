pub mod sandsim;

#[cfg(test)]
mod tests {
    use super::*;

    use sandsim::SandSim;

    #[test]
    fn test_part1() {
        let mut sim = SandSim::from_path("inputs/test.txt");
        assert_eq!(sim.sand_capacity(), 24);
    }

    #[test]
    fn test_part2() {
        let mut sim = SandSim::from_path("inputs/test.txt");
        assert_eq!(sim.sand_capacity_with_inf_floor(), 93);
    }
}
