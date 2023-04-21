
use std::fs;
use std::io::{BufRead,BufReader};

fn main() {
    let fname = "input.txt";
    let reader = BufReader::new(fs::File::open(fname).unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        //let hand: Vec[RPS]::new();
        for c in line.split_whitespace() {
            
        }
    }
}