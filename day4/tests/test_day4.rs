#[cfg(test)]
mod tests_day4 {

    use core::num;

    use day4::*;

    #[test]
    fn test_range_parse() {
        let range = Range::parse("2-4");
        assert_eq!(range.lo, 2);
        assert_eq!(range.hi, 4);
    }

    #[test]
    fn test_range_file_parse() {
        let ranges = parse_range_file("input_test.txt");

        assert_eq!(ranges[0].0.lo, 2);
        assert_eq!(ranges[0].0.hi, 4);
        assert_eq!(ranges[0].1.lo, 6);
        assert_eq!(ranges[0].1.hi, 8);

        assert_eq!(ranges[1].0.lo, 2);
        assert_eq!(ranges[1].0.hi, 3);
        assert_eq!(ranges[1].1.lo, 4);
        assert_eq!(ranges[1].1.hi, 5);
    }

    #[test]
    fn test_pt1() {
        let ranges = parse_range_file("input_test.txt");
        let num_contained = count_contained_ranges(&ranges);
        assert_eq!(num_contained, 2);
    }

    #[test]
    fn test_pt2() {
        let ranges = parse_range_file("input_test.txt");
        let num_contained = count_overlapping_ranges(&ranges);
        assert_eq!(num_contained, 4);
    }
}
