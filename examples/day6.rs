use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let mut file = File::open("inputs/day6.txt").expect("Couldn't find input file!");
    let mut input = String::new();
    file.read_to_string(&mut input).expect("couldn't read file");

    let mut total_everyone = 0;
    let mut total = 0;
    for group in input.split("\n\n") {
        println!("{}", group);

        let mut group_ans = HashMap::new();
        let mut members = 1;
        for c in group.chars() {
            if c != '\n' {
                match group_ans.get(&c) {
                    Some(n) => group_ans.insert(c, n + 1),
                    None => group_ans.insert(c, 1),
                };
            } else {
                members += 1;
            }
        }
        for (c, count) in group_ans.iter() {
            if count == &members {
                print!("{} ", c);
                total_everyone += 1;
            }
        }
        println!("\n");
        total += group_ans.len();
    }

    println!("Part One: {}", total);
    println!("Part Two: {}", total_everyone)
}
