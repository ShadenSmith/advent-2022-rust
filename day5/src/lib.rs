
use std::fmt;
use std::primitive::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;


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
            Some(_) => Some(Crate {id : caps?.get(1).unwrap().as_str().chars().collect::<Vec<_>>()[0] }),
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
        ShipState { stacks: stacks }
    }

    pub fn move_crates(&self, count: u32, from_stack: u32, to_stack: u32) -> Option<Crate> {
        None
    }
}

pub fn parse_crate_file(path: &str) -> () {
    let reader = BufReader::new(File::open(path).expect("File not found"));
    for line in reader.lines() {

    }
}