#[cfg(test)]
mod tests_day3 {

    use day3::*;

    #[test]
    fn test_find_shared_items() {
        //let chunks = vec!["abc".chars().collect(), "bww".chars().collect(), "jjb".chars().collect()];
        //assert_eq!(find_shared_items(chunks), vec!['b']);
    }

    #[test]
    fn test_rucksack_parse() {
        let sack = Rucksack::parse("abcdEFGH");
        assert_eq!(sack.left_sack(), "abcd");
        assert_eq!(sack.right_sack(), "EFGH");
    }

    #[test]
    fn test_rucksack_findshared() {
        let sack = Rucksack::parse("abXcdEFGHX");
        assert_eq!(sack.find_shared_item(), 'X');
    }

    #[test]
    fn test_pt1() {
        let sacks = parse_rucksacks("input_test.txt");
        assert_eq!(overlap_priority(&sacks), 157);
    }

    #[test]
    fn test_team_parse() {
        let teams = parse_teams("input_test.txt");
        assert_eq!(teams.len(), 2);

        assert_eq!(
            teams[0].group_a,
            "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            teams[1].group_c,
            "CrZsJsPPZsGzwwsLwLmpwMDw".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_pt2() {
        let teams = parse_teams("input_test.txt");
        assert_eq!(badge_priorities(&teams), 70);
    }
}
