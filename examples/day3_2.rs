use std::fs::File;
use std::io::{Read};

fn main() {

    let mut file = File::open("inputs/day3.txt").expect("day3.txt input not found!");
    let mut map = String::with_capacity(10336); // wc -c inputs/day3.txt
    
    file.read_to_string(&mut map).expect("Error reading file to string");

    let mut total = 1;
    for &(right, down) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let trees = tree_count(&map, right, down);
        println!("Right {}, down {}: {}", right, down, trees);
        total *= trees;
    }
    println!("Total product: {}", total);
}

fn tree_count(map: &str, right: usize, down: usize) -> usize {
    let mut position = 0;
    let mut count = 0;
    for line in map.lines().step_by(down) {
        
        if line.chars().nth(position).unwrap() == '#' {
            count += 1;
        }

        position += right;
        if position >= 31 {
            position -= 31;
        }
    }
    count
}