mod heightmap;

#[cfg(test)]
mod tests {
    use super::*;
    use heightmap::HeightMap;

    #[test]
    fn test_part1() {
        let hmap = HeightMap::from_path("input_test.txt");
        assert_eq!(hmap.count_visible_trees(), 21);
    }

    #[test]
    fn test_part2() {
        let hmap = HeightMap::from_path("input_test.txt");
        assert_eq!(hmap.max_scenic(), 8);
    }
}
