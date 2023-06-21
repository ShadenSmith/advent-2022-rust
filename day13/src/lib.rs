
pub mod packets;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(packets::num_misordered_packets("inputs/test.txt"), 13);
    }

}
