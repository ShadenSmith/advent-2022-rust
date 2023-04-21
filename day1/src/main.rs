pub mod day1 {

    use std::collections::BinaryHeap;
    use std::fs;
    use std::io::{BufRead, BufReader};

    pub fn sumtopk_elves(path: &str, k: i32) -> i32 {
        let reader = BufReader::new(fs::File::open(path).unwrap());

        let mut most_cals = 0;
        let mut curr_cals = 0;

        let mut top_calories = BinaryHeap::new();

        for line in reader.lines() {
            match line.unwrap().parse::<i32>() {
                Ok(calories) => {
                    curr_cals += calories;
                }

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

        let mut topk = 0;
        for _ in 0..k {
            topk += top_calories.pop().unwrap();
        }
        return topk;
    }
}

fn main() {
    let fname: String = String::from("day1/input.txt");
    let sumtop3 = day1::sumtopk_elves(&fname, 3);
    println!("Top 3: {sumtop3}");
}
