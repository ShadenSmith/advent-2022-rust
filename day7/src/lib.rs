#[allow(dead_code, unused_variables)]
mod babyfs;

#[cfg(test)]
mod tests {
    use crate::babyfs::fs::FileSystem;

    #[test]
    fn test_part1() {
        let fs = FileSystem::from_path("input_test.txt").unwrap();
        assert_eq!(fs.part1(), 95437);
    }

    #[test]
    fn test_part2() {
        let fs = FileSystem::from_path("input_test.txt").unwrap();
        assert_eq!(fs.part2(), 24933642);
    }
}
