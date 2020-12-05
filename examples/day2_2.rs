use std::fs::File;
use std::io;
use std::io::BufRead;

extern crate regex;

use regex::Regex;

fn main() {
    println!("{}", true ^ true);

    let data = io::BufReader::new(File::open("inputs/day2.txt").unwrap());

    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): (.*$)").unwrap();

    let mut valid = 0;
    for line in data.lines() {
        let line = line.unwrap();
        if let Some(groups) = re.captures(&line) {
            let min: usize = groups.get(1).unwrap().as_str().parse().unwrap();
            let max: usize = groups.get(2).unwrap().as_str().parse().unwrap();
            let rule = groups.get(3).unwrap().as_str().as_bytes()[0];
            let pw = groups.get(4).unwrap().as_str().as_bytes();

            /* a quick check confirms that max never exceeds the length of the string */
            if (rule == pw[min - 1]) ^ (rule == pw[max - 1]) {
                valid += 1;
                println!("\u{001B}[32m{}\u{001B}[0m", line.trim());
            } else {
                println!("\u{001B}[31m{}\u{001B}[0m", line.trim());
            }
        } else {
            break;
        }
    }
    println!("{} valid passwords", valid);
}
