
use std::fs;
use std::io::{BufRead,BufReader};
use std::collections::BinaryHeap;


fn main() {
    let fname = "input.txt";
    let reader = BufReader::new(fs::File::open(fname).unwrap());

    let mut most_cals = 0;
    let mut curr_cals = 0;

    let mut top_calories = BinaryHeap::new();

    for line in reader.lines() {
        match line.unwrap().parse::<i32>() {
            Ok(calories) => { 
                curr_cals += calories;
            },

            // Blank line, move to the next elf
            Err(_e) => {
                if curr_cals > most_cals {
                    most_cals = curr_cals;
                }

                top_calories.push(curr_cals);
                curr_cals = 0;
            }
        }
    }

    println!("most: {}", most_cals);

    let mut top3 = 0;
    for _ in 0..3 {
        top3 += top_calories.pop().unwrap();
    }

    println!("Top 3: {top3}");
}