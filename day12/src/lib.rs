pub mod rescue_map;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let map = rescue_map::RescueMap::from_path("inputs/test.txt");
        assert_eq!(map.shortest_path(), 31);
    }

    #[test]
    fn test_part2() {
        let map = rescue_map::RescueMap::from_path("inputs/test.txt");
        assert_eq!(map.shortest_path_scenic(), 29);
    }
}
