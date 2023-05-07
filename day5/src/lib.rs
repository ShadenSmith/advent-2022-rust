#![allow(unused, unused_imports)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::primitive::char;
use std::{fmt, vec};

use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Crate {
    pub id: char,
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.id)
    }
}

impl Crate {
    pub fn parse(inp: &str) -> Option<Crate> {
        lazy_static! {
            static ref RE_CRATE: Regex = Regex::new(r"\[([A-Z])\]").unwrap();
        }

        if RE_CRATE.captures_len() != 2 {
            panic!("Captured multiple crates in parse()");
        }

        let caps = RE_CRATE.captures(inp);
        match caps {
            Some(_) => Some(Crate {
                id: caps?.get(1).unwrap().as_str().chars().collect::<Vec<_>>()[0],
            }),
            _ => None,
        }
    }
}

pub struct ShipState {
    stacks: Vec<VecDeque<Crate>>,
}

impl ShipState {
    pub fn new(num_stacks: u32) -> ShipState {
        let mut stacks = Vec::new();
        for idx in 0..num_stacks {
            stacks.push(VecDeque::new())
        }
        ShipState { stacks }
    }

    pub fn move_crates(&mut self, count: u32, from_stack: u32, to_stack: u32) -> Option<Crate> {
        // Switch from 1-indexing to 0-indexing
        let from_stack_idx: usize = (from_stack - 1).try_into().unwrap();
        let to_stack_idx: usize = (to_stack - 1).try_into().unwrap();

        if from_stack_idx >= self.stacks.len() {
            panic!("Invalid source stack.");
        }
        if to_stack_idx >= self.stacks.len() {
            panic!("Invalid destination stack.");
        }

        if self.stacks[from_stack_idx].len() < count.try_into().unwrap() {
            panic!("Stack does not hold enough crates to move.");
        }

        for c in 0..count {
            let popped = self.stacks[from_stack_idx].pop_front().unwrap();
            self.stacks[to_stack_idx].push_front(popped);
        }

        None
    }

    pub fn parse_crate_line(line: &str) -> Vec<Option<Crate>> {
        let mut crates = Vec::new();

        let mut offset: usize = 0;
        while offset < line.len() {
            crates.push(Crate::parse(&line[offset..offset + 3]));
            offset += 4;
        }

        crates
    }

    pub fn from_path(path: &str) -> ShipState {
        let mut reader = BufReader::new(File::open(path).expect("File not found"));
        ShipState::from_file(&mut reader)
    }

    pub fn from_file(reader: &mut BufReader<File>) -> ShipState {
        //let mut reader = BufReader::new(File::open(path).expect("File not found"));

        let mut line_buf = String::new();

        let mut parsed_stacks: Vec<Vec<Option<Crate>>> = vec![];

        let mut parsing_init = true;
        while parsing_init {
            line_buf.clear();
            reader
                .read_line(&mut line_buf)
                .expect("Could not parse line.");

            let parsed_crates = ShipState::parse_crate_line(&line_buf);

            // Stop when we reach a line with no crates
            if !parsed_crates.iter().any(|c| c.is_some()) {
                parsing_init = false;
                break;
            }
            parsed_stacks.push(parsed_crates);
        }

        // Eat the next line of stack numbers
        line_buf.clear();
        reader
            .read_line(&mut line_buf)
            .expect("Could not parse line.");

        if parsed_stacks.is_empty() {
            return ShipState::new(0);
        }

        let mut ship = ShipState::new(parsed_stacks[0].len().try_into().unwrap());
        for parsed_line in parsed_stacks.iter() {
            // Add any parsed crates to the end of the stack
            for (stack_idx, parsed) in parsed_line.iter().enumerate() {
                if let Some(c) = parsed {
                    ship.stacks[stack_idx].push_back(c.clone());
                }
            }
        }

        ship
    }

    pub fn num_stacks(&self) -> usize {
        self.stacks.len()
    }

    pub fn top_stacks(&self) -> String {
        let mut top = String::new();

        for stack in self.stacks.iter() {
            let top_stack = stack.get(0);

            if let Some(c) = top_stack {
                top.push(c.id)
            } else {
                top.push(' ');
            }
        }

        top
    }

    pub fn execute(&mut self, steps: &[Step]) {
        for step in steps.iter() {
            self.move_crates(step.count, step.from, step.to);
        }
    }
}

#[derive(Debug)]
pub struct Step {
    pub count: u32,
    pub from: u32,
    pub to: u32,
}

pub fn parse_steps(reader: &mut BufReader<File>) -> Vec<Step> {
    lazy_static! {
        static ref RE_CRATE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    }

    let mut steps = Vec::new();

    for line in reader.lines() {
        if let Some(caps) = RE_CRATE.captures(&line.unwrap()) {
            let count: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let from: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let to: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
            steps.push(Step { count, from, to })
        }
    }

    steps
}

pub fn parse_crate_file(path: &str) -> (ShipState, Vec<Step>) {
    let mut reader = BufReader::new(File::open(path).expect("File not found"));

    let ship = ShipState::from_file(&mut reader);
    let steps = parse_steps(&mut reader);

    (ship, steps)
}

#[cfg(tests)]
mod tests_day5 {
    use super::*;
}
