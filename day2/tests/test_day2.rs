#[cfg(test)]
mod tests {
    use day2::*;

    #[test]
    fn test_rps_parse() {
        assert_eq!(RPS::new("A").value(), 1);
        assert_eq!(RPS::new("B").value(), 2);
        assert_eq!(RPS::new("C").value(), 3);
        assert_eq!(RPS::new("X").value(), 1);
        assert_eq!(RPS::new("Y").value(), 2);
        assert_eq!(RPS::new("Z").value(), 3);
    }

    #[test]
    fn test_day2() {
        let guide = parse_strategy_guide("input_test.txt");
        assert_eq!(score_strategy_guide(&guide), 15);
    }
}
