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

pub fn find_start_marker(msg: &str, window_size: usize) -> usize {
    if msg.len() < window_size {
        return msg.len();
    }

    for offset in 0..msg.len() - window_size {
        if !repeats_in_window(&msg[offset..offset + window_size]) {
            return offset + window_size;
        }
    }

    msg.len()
}

pub fn find_packet_start(msg: &str) -> usize {
    find_start_marker(msg, 4)
}

pub fn find_message_start(msg: &str) -> usize {
    find_start_marker(msg, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pt1() {
        assert_eq!(find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(find_packet_start("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_pt2() {
        assert_eq!(find_message_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_message_start("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_message_start("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_message_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_message_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
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
