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

#[cfg(test)]
mod tests {
    use super::*;
}
