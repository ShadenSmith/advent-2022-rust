#[cfg(test)]
mod tests_day3 {

    use day3::*;

    #[test]
    fn test_rucksack_parse() {
        let sack = Rucksack::parse("abcdEFGH");
        assert_eq!(sack.left, "abcd");
        assert_eq!(sack.right, "EFGH");
    }

    #[test]
    fn test_pt1() {
        let x = 3;
        assert_eq!(x, 157);
    }
}
