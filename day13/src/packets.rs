use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum PacketElem {
    List(Vec<PacketElem>),
    Val(i64),
}

struct Packet {
    data: PacketElem,
}

impl Packet {
    pub fn empty() -> Self {
        Packet {
            data: PacketElem::List(vec![]),
        }
    }

    pub fn parse(line: &str) -> Self {
        fn parse_list(subline: &str) -> Packet {
            Packet::empty()
        }

        println!("Parsing {line}");
        for tok in line.chars() {
            println!("  tok: '{tok}'");
        }

        Packet::empty()
    }

    pub fn misordered(left: &Packet, right: &Packet) -> bool {
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

        let left = Packet::parse(&buf_left);
        let right = Packet::parse(&buf_right);

        if Packet::misordered(&left, &right) {
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

    #[test]
    fn test_packet_parse_single() {
        assert_eq!(
            Packet::parse("[1]").data,
            PacketElem::List(vec![PacketElem::Val(1)])
        );
    }

    #[test]
    fn test_packet_parse_list() {
        assert_eq!(
            Packet::parse("[1,2,3]").data,
            PacketElem::List(vec![
                PacketElem::Val(1),
                PacketElem::Val(2),
                PacketElem::Val(3),
            ])
        );
    }
}
