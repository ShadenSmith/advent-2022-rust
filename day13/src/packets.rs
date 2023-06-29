use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum PacketElem {
    List(Vec<PacketElem>),
    Val(i64),
}

struct PacketLegacy {
    data: PacketElem,
}

impl PacketLegacy {
    pub fn empty() -> Self {
        PacketLegacy {
            data: PacketElem::List(vec![]),
        }
    }

    pub fn parse(line: &str) -> Self {
        fn parse_list(subline: &str) -> PacketLegacy {
            PacketLegacy::empty()
        }

        PacketLegacy::empty()
    }

    pub fn misordered(left: &PacketLegacy, right: &PacketLegacy) -> bool {
        false
    }
}

pub fn num_misordered_packets(path: &str) -> usize {
    let mut reader = BufReader::new(File::open(path).expect("Could not open file."));

    let mut buf_left = String::new();
    let mut buf_right = String::new();

    let mut count = 0;

    loop {
        let res_left = reader.read_line(&mut buf_left);
        let res_right = reader.read_line(&mut buf_right);
        if res_left.is_err() || res_right.is_err() {
            break;
        }
        if res_left.unwrap() == 0 || res_right.unwrap() == 0 {
            break;
        }

        let left = PacketLegacy::parse(&buf_left);
        let right = PacketLegacy::parse(&buf_right);

        if PacketLegacy::misordered(&left, &right) {
            count += 1;
        }

        buf_left.clear();
        buf_right.clear();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
}
