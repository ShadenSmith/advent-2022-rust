#[cfg(test)]
mod tests {
    use day2::*;

    #[test]
    fn test_rps_parse() {
        assert_eq!(State::parse("A").value(), 1);
        assert_eq!(State::parse("B").value(), 2);
        assert_eq!(State::parse("C").value(), 3);
        assert_eq!(State::parse("X").value(), 1);
        assert_eq!(State::parse("Y").value(), 2);
        assert_eq!(State::parse("Z").value(), 3);
    }

    #[test]
    fn test_day2_pt1() {
        let guide = parse_strategy_guide("input_test.txt");
        assert_eq!(score_strategy_guide(&guide), 15);
    }

    #[test]
    fn test_day2_pt2() {
        let guide = parse_strategy_guide("input_test.txt");
        assert_eq!(score_strategy_guide_pt2(&guide), 12);
    }
}
