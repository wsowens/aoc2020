use std::fs::File;
use std::io;
use std::io::BufRead;


extern crate regex;

use regex::Regex;

fn main() {
    let data = io::BufReader::new(File::open("inputs/day2.txt").unwrap());

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*$)").unwrap();

    let mut valid = 0;
    for line in data.lines() {
        let line = line.unwrap();
        if let Some(groups) = re.captures(&line) {
            let min: u32 = groups.get(1).unwrap().as_str().parse().unwrap();
            let max: u32 = groups.get(2).unwrap().as_str().parse().unwrap();
            let rule = groups.get(3).unwrap().as_str().as_bytes()[0];
            let pw = groups.get(4).unwrap().as_str();

            let mut count = 0;
            for u in pw.bytes() {
                if u == rule {
                    count += 1;
                }
            }
            if min <= count && count <= max {
                valid += 1;
                println!("\u{001B}[32m{}\u{001B}[0m", line.trim());
            } else {
                println!("\u{001B}[31m{}\u{001B}[0m", line.trim());
            }
        }
        else {
            break;
        }
    }
    println!("{} valid passwords", valid);
}