fn repeats_in_window(window: &str) -> bool {
    let cmp = |a, b| window[a..a + 1] == window[b..b + 1];

    for left in 0..window.len() - 1 {
        for right in left + 1..window.len() {
            if cmp(left, right) {
                return true;
            }
        }
    }

    false
}

pub fn find_start_marker(msg: &str) -> usize {
    if msg.len() < 4 {
        return msg.len();
    }

    for offset in 0..msg.len() - 4 {
        if !repeats_in_window(&msg[offset..offset + 4]) {
            return offset + 4;
        }
    }

    msg.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(find_start_marker("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_start_marker("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_start_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_start_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_window() {
        assert_eq!(repeats_in_window("abcd"), false);
        assert_eq!(repeats_in_window("aaaa"), true);
        assert_eq!(repeats_in_window("abca"), true);
        assert_eq!(repeats_in_window("abcc"), true);
        assert_eq!(repeats_in_window("abbc"), true);
    }
}
