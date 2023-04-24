
#[cfg(test)]
mod tests_day5 {
    use day5::*;

    #[test]
    fn crate_parse() {
        assert_eq!(Crate::parse("[B]").unwrap().id, 'B');
        assert!(Crate::parse("   ").is_none());
    }

    #[test]
    fn test_pt1() {
        todo!()
    }
}