use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("inputs/day5.txt").expect("Couldn't find input file!");

    let mut max = 0;

    let mut nums = vec![];

    for line in io::BufReader::new(file).lines() {
        let line = line.expect("couldnt read line");
        let pass = parse_pass(&line);
        nums.push(pass);
        if pass > max {
            max = pass;
        }
    }
    nums.sort();
    let mut prev = None;
    for value in nums {
        if let Some(p) = prev {
            if p != value - 1 {
                println!("missing value: {}, {}", p, value);
            }
        }
        prev = Some(value);
    }
    println!("max: {}", max);
}

fn parse_pass(pass: &str) -> u32 {
    let value: String = pass
        .chars()
        .map(|c| c == 'B' || c == 'R')
        .map(|b| if b { '1' } else { '0' })
        .collect();
    u32::from_str_radix(&value, 2).unwrap()
}
