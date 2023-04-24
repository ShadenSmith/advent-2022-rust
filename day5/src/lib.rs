
use std::fmt;
use std::primitive::char;

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
