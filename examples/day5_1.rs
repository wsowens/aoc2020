use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("inputs/day5.txt").expect("Couldn't find input file!");

    let mut max = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.expect("couldnt read line");
        let pass = parse_pass(&line);
        println!("{}: {}", line, pass);
        if pass > max {
            max = pass;
        }
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
