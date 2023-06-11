use crate::worry::Worry;

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

#[derive(PartialEq, Debug)]
enum InspectOperand {
    Old,
    Val(Worry),
}

impl InspectOperand {
    pub fn parse(token: &str) -> Self {
        match token {
            "old" => InspectOperand::Old,
            val => InspectOperand::Val(Worry::parse(token)),
        }
    }
}

#[derive(PartialEq, Debug)]
enum InspectOp {
    Add,
    Mul,
}

impl InspectOp {
    pub fn parse(token: &str) -> Self {
        match token.trim() {
            "+" => InspectOp::Add,
            "*" => InspectOp::Mul,
            _ => panic!("Could not parse op."),
        }
    }
}

struct ItemInspection {
    pub op: InspectOp,
    pub operands: (InspectOperand, InspectOperand),

    pub div_test: Worry,
    pub destinations: (usize, usize),
}

impl ItemInspection {
    pub fn inspect(&self, old: Worry) -> Worry {
        let left = match &self.operands.0 {
            InspectOperand::Old => old.clone(),
            InspectOperand::Val(x) => x.clone(),
        };
        let right = match &self.operands.1 {
            InspectOperand::Old => old.clone(),
            InspectOperand::Val(x) => x.clone(),
        };

        match self.op {
            InspectOp::Add => left + right,
            InspectOp::Mul => left * right,
        }
    }

    fn parse_operation(line: &str) -> (InspectOp, (InspectOperand, InspectOperand)) {
        lazy_static! {
            static ref INSPECT_OP_RE: Regex =
                Regex::new(r#"\w*Operation: new = (.+) (.+) (.+)"#).expect("Invalid regex.");
        }

        let cap = INSPECT_OP_RE.captures(&line).unwrap();

        let op = InspectOp::parse(cap.get(2).as_ref().unwrap().as_str());
        let left = InspectOperand::parse(cap.get(1).as_ref().unwrap().as_str());
        let right = InspectOperand::parse(cap.get(3).as_ref().unwrap().as_str());

        (op, (left, right))
    }

    fn parse_div_test(line: &str) -> Worry {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"\w*Test: divisible by (.+)"#).expect("Invalid regex.");
        }

        let cap = RE
            .captures(&line)
            .expect("Could not parse divisibility test");

        Worry(
            cap.get(1)
                .as_ref()
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap(),
        )
    }

    fn parse_destinations(lines: &[&str]) -> (usize, usize) {
        let t_line = lines[0];
        let f_line = lines[1];

        let t_dest = t_line
            .split("monkey ")
            .skip(1)
            .take(1)
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let f_dest = f_line
            .split("monkey ")
            .skip(1)
            .take(1)
            .last()
            .unwrap()
            .parse()
            .unwrap();

        (t_dest, f_dest)
    }

    pub fn from_notes(lines: &[&str]) -> Self {
        let (op, operands) = ItemInspection::parse_operation(&lines[0]);
        let div_test = ItemInspection::parse_div_test(&lines[1]);
        let destinations = ItemInspection::parse_destinations(&lines[2..=3]);

        ItemInspection {
            op,
            operands,
            div_test,
            destinations,
        }
    }
}

struct Monkey {
    pub items: VecDeque<Worry>,
    pub inspection: ItemInspection,
}

impl Monkey {
    pub fn new(items: &[Worry], inspect: ItemInspection) -> Self {
        Monkey {
            items: items.iter().cloned().collect(),
            inspection: inspect,
        }
    }

    pub fn go(&mut self) {
        //while let Some(worry_level) = self.items.pop_front() {
    }

    fn parse_item_log(log_line: &str) -> Vec<Worry> {
        let items_reg = Regex::new(r#"\w*Starting items: (.*)"#).unwrap();

        let cap = items_reg.captures(&log_line).unwrap();
        let item_str = cap.get(1).as_ref().unwrap().as_str();
        let items = item_str
            .split(", ")
            .into_iter()
            .map(|s| Worry(s.parse::<i64>().unwrap()))
            .collect();
        items
    }

    pub fn from_notes(notes: &[&str]) -> Self {
        Monkey::new(
            &Monkey::parse_item_log(&notes[0]),
            ItemInspection::from_notes(&notes[1..notes.len()]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monkey_parse() {
        let notes = vec![
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
        ];

        let m = Monkey::from_notes(&notes);
        assert_eq!(m.items, vec![Worry(79), Worry(98)]);
        assert_eq!(m.inspection.op, InspectOp::Mul);
        assert_eq!(m.inspection.operands.0, InspectOperand::Old);
        assert_eq!(m.inspection.operands.1, InspectOperand::Val(Worry(19)));
        assert_eq!(m.inspection.destinations.0, 2);
        assert_eq!(m.inspection.destinations.1, 3);
    }
}
