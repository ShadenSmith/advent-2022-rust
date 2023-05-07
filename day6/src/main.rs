use day6::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let path = "day6/input.txt";
    let mut reader = BufReader::new(File::open(path).expect("File not found"));

    let mut msg = String::new();
    if let Err(error) = reader.read_line(&mut msg) {
        panic!("Read failed: {}", error);
    }

    println!("part 1: {}", find_start_marker(&msg));
}
