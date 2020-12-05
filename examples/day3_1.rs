use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("inputs/day3.txt").expect("day3.txt input not found!");
    let mut position = 0;
    let mut count = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();

        // you could also call line.chars().cycle().nth()
        if line.chars().nth(position).unwrap() == '#' {
            count += 1;
        }

        // my puzzle is 31 long, you might need to change this
        position += 3;
        // my CS professor once told me to avoid integer division as much as possible
        // but is bad branch prediction just as bad?
        if position >= 31 {
            position -= 31;
        }
    }
    println!("Total Trees: {}", count);
}
