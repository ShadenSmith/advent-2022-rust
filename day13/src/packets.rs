use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::packet::{are_packets_ordered, Packet};

pub fn num_misordered_packets(path: &str) -> usize {
    let mut reader = BufReader::new(File::open(path).expect("Could not open file."));

    let mut buf_left = String::new();
    let mut buf_right = String::new();

    let mut count = 0;

    let mut idx = 1;

    loop {
        let res_left = reader.read_line(&mut buf_left);
        let res_right = reader.read_line(&mut buf_right);
        if res_left.is_err() || res_right.is_err() {
            break;
        }
        if res_left.unwrap() == 0 || res_right.unwrap() == 0 {
            break;
        }

        let left: Packet = buf_left
            .trim_end()
            .parse()
            .expect("Could not parse packet.");
        let right: Packet = buf_right
            .trim_end()
            .parse()
            .expect("Could not parse packet.");

        if are_packets_ordered(&left, &right) {
            count += idx;
        }

        buf_left.clear();
        buf_right.clear();

        // Read next empty line between pairs
        _ = reader.read_line(&mut buf_left);
        buf_left.clear();
        idx += 1;
    }

    count
}

pub fn part2(path: &str) -> usize {
    let reader = BufReader::new(File::open(path).expect("Could not open file."));

    let mut parsed: Vec<Packet> = reader
        .lines()
        .filter(|l| l.as_ref().unwrap().len() > 0)
        .map(|l| l.unwrap().trim_end().parse().unwrap())
        .collect();

    let markers: Vec<Packet> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];

    parsed.push("[[2]]".parse().unwrap());
    parsed.push("[[6]]".parse().unwrap());

    parsed.sort_by(|a, b| {
        if are_packets_ordered(a, b) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });

    let mut marker_idxs = Vec::new();
    for (idx, p) in parsed.iter().enumerate() {
        if markers.contains(p) {
            marker_idxs.push(idx + 1);
        }
    }
    println!("{marker_idxs:?}");

    marker_idxs.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(num_misordered_packets("inputs/test.txt"), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("inputs/test.txt"), 140);
    }
}
