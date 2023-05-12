#[allow(dead_code, unused_variables)]
mod babyfs;

#[cfg(test)]
mod tests {
    use crate::babyfs::fs::FileSystem;

    #[test]
    fn test_part1() {
        let fs = FileSystem::from_path("input_test.txt").unwrap();
        todo!();
    }
}
