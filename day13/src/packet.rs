use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::{AsChar, Finish, IResult};

use std::cmp::Ordering;

use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Packet {
    List(Vec<Packet>),
    Val(i64),
}

impl Packet {
    pub fn empty() -> Self {
        Packet::List(vec![])
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = parse_list_atom(s).finish();
        if let Ok((_, packet)) = parsed {
            Ok(packet)
        } else {
            Err(parsed.err().unwrap().to_string())
        }
    }
}

pub fn are_packets_ordered(left: &Packet, right: &Packet) -> bool {
    use Packet::{List, Val};

    fn inner(left_node: &Packet, right_node: &Packet) -> Ordering {
        match (left_node, right_node) {
            (Val(a), Val(b)) => a.cmp(b),
            (Val(a), List(_)) => inner(&List(vec![Val(*a)]), right_node),
            (List(_), Val(b)) => inner(left_node, &List(vec![Val(*b)])),
            (List(avs), List(bvs)) => {
                // Walk the lists and compare
                let mut left_iter = avs.iter();
                let mut right_iter = bvs.iter();

                loop {
                    let a = left_iter.next();
                    let b = right_iter.next();

                    match (a, b) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(a_node), Some(b_node)) => {
                            let eval = inner(a_node, b_node);
                            if eval == Ordering::Equal {
                                // Move to the next pair of elems
                                continue;
                            }
                            return eval;
                        }
                    }
                }
            }
        }
    }

    match inner(left, right) {
        Ordering::Less => true,
        Ordering::Greater => false,
        Ordering::Equal => unreachable!(),
    }
}

fn parse_int_atom(input: &str) -> IResult<&str, Packet> {
    let parser = take_while1(AsChar::is_dec_digit);
    match parser(input) {
        Ok((rest, parsed_atom)) => Ok((
            rest,
            Packet::Val(parsed_atom.parse().expect("Could not parse integer.")),
        )),
        Err(x) => Err(x),
    }
}

fn parse_list_empty(input: &str) -> IResult<&str, Packet> {
    let empty_list = tag("[]");
    match empty_list(input) {
        Ok((rest, _)) => Ok((rest, Packet::empty())),
        Err(x) => Err(x),
    }
}

fn parse_list_single(input: &str) -> IResult<&str, Packet> {
    // parse: [3]
    let mut parser = delimited(tag("["), parse_int_atom, tag("]"));
    match parser(input) {
        Ok((rest, atom)) => Ok((rest, Packet::List(vec![atom]))),
        Err(x) => Err(x),
    }
}

fn parse_list_atom(input: &str) -> IResult<&str, Packet> {
    // Empty or single-item list
    let mut base_parser = alt((parse_list_empty, parse_list_single));
    if let Ok((rest, parsed)) = base_parser(input) {
        return Ok((rest, parsed));
    }

    // Recursive case: match a comma-separated string of ints and Packets
    let mut multi_parser = delimited(
        tag("["),
        separated_list1(
            tag(","),
            alt((
                parse_int_atom,
                parse_list_empty,
                parse_list_single,
                parse_list_atom,
            )),
        ),
        tag("]"),
    );
    match multi_parser(input) {
        Ok((rest, atoms)) => Ok((rest, Packet::List(atoms))),
        Err(x) => Err(x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int_atom() {
        assert_eq!(parse_int_atom("123"), Ok(("", Packet::Val(123))));
        assert_eq!(parse_int_atom("123)"), Ok((")", Packet::Val(123))));
    }

    #[test]
    fn test_parse_int_invalid() {
        assert!(parse_int_atom("abc").is_err());
    }

    #[test]
    fn test_parse_list_empty() {
        assert_eq!(parse_list_empty("[]"), Ok(("", Packet::empty())));
        assert_eq!(parse_list_atom("[]"), Ok(("", Packet::empty())));
    }

    #[test]
    fn test_parse_list_single() {
        assert_eq!(
            parse_list_atom("[3]"),
            Ok(("", Packet::List(vec![Packet::Val(3)])))
        );

        assert_eq!(
            parse_list_atom(r"[[3]]"),
            Ok(("", Packet::List(vec![Packet::List(vec![Packet::Val(3)])])))
        );

        assert_eq!(
            parse_list_atom(r"[[]]"),
            Ok(("", Packet::List(vec![Packet::List(vec![])])))
        );
    }

    #[test]
    fn test_parse_list_multi() {
        assert_eq!(
            parse_list_atom("[3,4]"),
            Ok(("", Packet::List(vec![Packet::Val(3), Packet::Val(4)])))
        );
    }

    #[test]
    fn test_parse_list_nested() {
        assert_eq!(
            parse_list_atom("[3,[1,[],8],[[1]]]"),
            Ok((
                "",
                Packet::List(vec![
                    Packet::Val(3),
                    Packet::List(vec![Packet::Val(1), Packet::List(vec![]), Packet::Val(8),]),
                    Packet::List(vec![Packet::List(vec![Packet::Val(1)])]),
                ])
            ))
        );
    }

    #[test]
    fn test_parse_packet() {
        assert_eq!(
            Ok(Packet::List(vec![Packet::Val(3), Packet::Val(4)])),
            "[3,4]".parse()
        );
    }

    #[test]
    fn test_parse_list_invalid() {
        assert!(parse_list_atom("[2,3,[2]").is_err());
        assert!(parse_list_atom("[[2],3,,[2]]").is_err());

        // TODO: this incorrectly passes
        // assert!(parse_list_atom("[2],3,[2]").is_err());
    }

    #[test]
    fn test_ordered_ints() {
        let left: Packet = "[1,1,3,1,1]".parse().unwrap();
        let right: Packet = "[1,1,5,1,1]".parse().unwrap();
        assert_eq!(are_packets_ordered(&left, &right), true);
    }

    #[test]
    fn test_ordered_mixed1() {
        let left: Packet = "[[1],[2,3,4]]".parse().unwrap();
        let right: Packet = "[[1],4]".parse().unwrap();
        assert_eq!(are_packets_ordered(&left, &right), true);
    }

    #[test]
    fn test_ordered_mixed2() {
        let left: Packet = "[9]".parse().unwrap();
        let right: Packet = "[[8,7,6]]".parse().unwrap();
        assert_eq!(are_packets_ordered(&left, &right), false);
    }

    #[test]
    fn test_ordered_merge() {
        let left: Packet = "[7,7,7,7]".parse().unwrap();
        let right: Packet = "[7,7,7]".parse().unwrap();
        assert_eq!(are_packets_ordered(&left, &right), false);
    }
}
