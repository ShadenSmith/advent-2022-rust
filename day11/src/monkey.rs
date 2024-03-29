use crate::worry::Worry;

use lazy_static::lazy_static;
use regex::Regex;
use std::{borrow::BorrowMut, collections::VecDeque};

#[derive(PartialEq, Debug, Clone)]
pub enum InspectOperand {
    Old,
    Val(Worry),
}

impl InspectOperand {
    pub fn parse(token: &str) -> Self {
        match token {
            "old" => InspectOperand::Old,
            _ => InspectOperand::Val(Worry::parse(token)),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum InspectOp {
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

#[derive(Debug, Clone)]
pub struct ItemInspection {
    pub op: InspectOp,
    pub operands: (InspectOperand, InspectOperand),

    pub div_test: Worry,
    pub destinations: (usize, usize),
}

impl ItemInspection {
    pub fn inspect(&self, old: Worry, div_level: &Worry) -> (Worry, usize) {
        let left = match &self.operands.0 {
            InspectOperand::Old => old.clone(),
            InspectOperand::Val(x) => x.clone(),
        };
        let right = match &self.operands.1 {
            InspectOperand::Old => old,
            InspectOperand::Val(x) => x.clone(),
        };

        let post_op = match self.op {
            InspectOp::Add => left + right,
            InspectOp::Mul => left * right,
        };

        let bored = post_op / div_level.clone();

        let monkey_dest = if bored.0 % self.div_test.0 == 0 {
            self.destinations.0
        } else {
            self.destinations.1
        };

        (bored, monkey_dest)
    }

    fn parse_operation(line: &str) -> (InspectOp, (InspectOperand, InspectOperand)) {
        lazy_static! {
            static ref INSPECT_OP_RE: Regex =
                Regex::new(r#"\w*Operation: new = (.+) (.+) (.+)"#).expect("Invalid regex.");
        }

        let cap = INSPECT_OP_RE.captures(line).unwrap();

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
            .captures(line)
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
        let (op, operands) = ItemInspection::parse_operation(lines[0]);
        let div_test = ItemInspection::parse_div_test(lines[1]);
        let destinations = ItemInspection::parse_destinations(&lines[2..=3]);

        ItemInspection {
            op,
            operands,
            div_test,
            destinations,
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
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

    fn parse_item_log(log_line: &str) -> Vec<Worry> {
        let items_reg = Regex::new(r#"\w*Starting items: (.*)"#).unwrap();

        let cap = items_reg.captures(log_line).unwrap();
        let item_str = cap.get(1).as_ref().unwrap().as_str();
        let items = item_str
            .split(", ")
            .map(|s| Worry(s.parse::<i64>().unwrap()))
            .collect();
        items
    }

    pub fn from_notes(notes: &[&str]) -> Self {
        Monkey::new(
            &Monkey::parse_item_log(notes[0]),
            ItemInspection::from_notes(&notes[1..notes.len()]),
        )
    }
    pub fn notes_len() -> usize {
        5
    }
}

#[derive(Debug)]
pub struct MonkeySystem {
    monkeys: Vec<Monkey>,
}

impl MonkeySystem {
    pub fn default() -> Self {
        MonkeySystem { monkeys: vec![] }
    }

    pub fn from_notes(lines: &[&str]) -> Self {
        let mut monkeys = vec![];
        let mut idx = 0;
        while idx < lines.len() {
            idx += 1; // "Monkey: i"
            if lines.len() - idx < Monkey::notes_len() {
                panic!("too few lines!");
            }

            monkeys.push(Monkey::from_notes(&lines[idx..idx + Monkey::notes_len()]));

            idx += Monkey::notes_len();
            idx += 1; // empty line
        }

        MonkeySystem { monkeys }
    }

    pub fn from_path(path: &str) -> Self {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let reader = BufReader::new(File::open(path).expect("Could not open file."));

        let ls = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        let ls_refs: Vec<&str> = ls.iter().map(|s| -> &str { s }).collect();
        MonkeySystem::from_notes(&ls_refs)
    }

    pub fn monkey_business(&mut self, rounds: usize, div_level: Worry) -> usize {
        let mut inspect_count: Vec<usize> = Vec::with_capacity(self.monkeys.len());
        for _ in 0..self.monkeys.len() {
            inspect_count.push(0);
        }

        // We just want a multiple of all the div_tests
        let global_worry_div: i64 = self
            .monkeys
            .iter()
            .map(|m| m.inspection.div_test.0)
            .product();

        for _round in 0..rounds {
            for curr_monkey in 0..self.monkeys.len() {
                let inspection = self.monkeys[curr_monkey].inspection.clone();
                while let Some(old_worry) = self.monkeys[curr_monkey].borrow_mut().items.pop_front()
                {
                    inspect_count[curr_monkey] += 1;

                    let (new_worry, dest) = inspection.inspect(old_worry, &div_level);

                    // This won't affect routing decisions
                    let scaled_worry = new_worry.0 % global_worry_div;

                    // send to next monkey
                    self.monkeys[dest]
                        .borrow_mut()
                        .items
                        .push_back(Worry(scaled_worry));
                }
            }
        }

        // return product of top 2 largest counts
        inspect_count.sort();
        inspect_count[inspect_count.len() - 1] * inspect_count[inspect_count.len() - 2]
    }

    pub fn print_items(&self) {
        for (idx, m) in self.monkeys.iter().enumerate() {
            let items = &m.items;
            println!("Monkey {idx}: {items:?}");
        }
        println!();
        println!();
    }
}

impl<'a> MonkeySystem {
    pub fn get(&'a self, idx: usize) -> &'a Monkey {
        &self.monkeys[idx]
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
        assert_eq!(m.inspection.div_test.0, 23);
        assert_eq!(m.inspection.destinations.0, 2);
        assert_eq!(m.inspection.destinations.1, 3);
    }

    #[test]
    fn monkey_system_parse() {
        let ms = MonkeySystem::from_path("inputs/test_part1.txt");
        assert_eq!(ms.get(0).items, vec![Worry(79), Worry(98)]);
        assert_eq!(ms.get(0).inspection.op, InspectOp::Mul);
        assert_eq!(ms.get(0).inspection.operands.0, InspectOperand::Old);
        assert_eq!(
            ms.get(0).inspection.operands.1,
            InspectOperand::Val(Worry(19))
        );
        assert_eq!(ms.get(0).inspection.div_test.0, 23);
        assert_eq!(ms.get(0).inspection.destinations.0, 2);
        assert_eq!(ms.get(0).inspection.destinations.1, 3);

        assert_eq!(
            ms.get(1).items,
            vec![Worry(54), Worry(65), Worry(75), Worry(74)]
        );
        assert_eq!(ms.get(1).inspection.op, InspectOp::Add);
        assert_eq!(ms.get(1).inspection.operands.0, InspectOperand::Old);
        assert_eq!(
            ms.get(1).inspection.operands.1,
            InspectOperand::Val(Worry(6))
        );
        assert_eq!(ms.get(1).inspection.div_test.0, 19);
        assert_eq!(ms.get(1).inspection.destinations.0, 2);
        assert_eq!(ms.get(1).inspection.destinations.1, 0);

        assert_eq!(ms.get(2).items, vec![Worry(79), Worry(60), Worry(97)]);
        assert_eq!(ms.get(2).inspection.op, InspectOp::Mul);
        assert_eq!(ms.get(2).inspection.operands.0, InspectOperand::Old);
        assert_eq!(ms.get(2).inspection.operands.1, InspectOperand::Old);
        assert_eq!(ms.get(2).inspection.div_test.0, 13);
        assert_eq!(ms.get(2).inspection.destinations.0, 1);
        assert_eq!(ms.get(2).inspection.destinations.1, 3);

        assert_eq!(ms.get(3).items, vec![Worry(74)]);
        assert_eq!(ms.get(3).inspection.op, InspectOp::Add);
        assert_eq!(ms.get(3).inspection.operands.0, InspectOperand::Old);
        assert_eq!(
            ms.get(3).inspection.operands.1,
            InspectOperand::Val(Worry(3))
        );
        assert_eq!(ms.get(3).inspection.div_test.0, 17);
        assert_eq!(ms.get(3).inspection.destinations.0, 0);
        assert_eq!(ms.get(3).inspection.destinations.1, 1);
    }

    #[test]
    fn test_part_1() {
        let mut ms = MonkeySystem::from_path("inputs/test_part1.txt");
        assert_eq!(ms.monkey_business(20, Worry(3)), 10605);
    }

    #[test]
    fn test_part_2() {
        let mut ms = MonkeySystem::from_path("inputs/test_part1.txt");
        assert_eq!(ms.monkey_business(10_000, Worry(1)), 2713310158);
    }
}
