#[cfg(test)]
mod tests_day5 {
    use day5::*;

    use std::iter::zip;

    #[test]
    fn crate_parse() {
        assert_eq!(Crate::parse("[B]").unwrap().id, 'B');
        assert!(Crate::parse("   ").is_none());
    }

    #[test]
    fn crate_parse_line() {
        fn do_compare(inp: &str, expected: &[Option<char>]) -> bool {
            let parsed = ShipState::parse_crate_line(inp);

            for (mine, test) in zip(&parsed, expected) {
                assert_eq!(mine.is_some(), test.is_some());
                if mine.is_some() {
                    assert_eq!(mine.as_ref().unwrap().id, test.unwrap());
                }
            }
            true
        }

        do_compare("    [D]    ", &vec![None, Some('D'), None]);
        do_compare("[N] [C]    ", &vec![Some('N'), Some('C'), None]);
        do_compare("[Z] [M] [P]", &vec![Some('Z'), Some('M'), Some('P')]);
    }

    #[test]
    fn test_parse_ship() {
        let ship = ShipState::from_path("input_test.txt");

        assert_eq!(ship.num_stacks(), 3);
        assert_eq!(ship.top_stacks(), "NDP");
    }

    #[test]
    fn test_move_crates() {
        let mut ship = ShipState::from_path("input_test.txt");
        ship.move_crates(2, 2, 3);
        assert_eq!(ship.top_stacks(), "NMC");
    }

    #[test]
    fn test_pt1() {
        let (mut ship, steps) = parse_crate_file("input_test.txt");

        for step in steps.iter() {
            ship.move_crates(step.count, step.from, step.to);
        }

        assert_eq!(ship.top_stacks(), "CMZ");
    }
}
