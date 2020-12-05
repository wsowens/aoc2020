use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

fn main() {
    let mut file = File::open("inputs/day4.txt").expect("Couldn't find input file!");
    let mut input = String::with_capacity(21663); // wc -c inputs/day4.txt
    file.read_to_string(&mut input).expect("couldn't read file");

    match parse_file(&input) {
        Ok((valid, total)) => println!("Valid passports: {}/{}", valid, total),
        Err(value) => println!("Error: {}", value),
    }
}

fn parse_file(input: &str) -> Result<(usize, usize), String> {
    let mut count = 0;
    let mut total = 0;
    let mut passport: Passport = Default::default();

    for field in input.split(&[' ', '\n'][..]) {
        match field {
            "" => {
                // end of passport, check this one and make a new one
                total += 1;
                if passport.valid() {
                    count += 1;
                    println!("✅");
                } else {
                    println!("❌");
                }
                passport = Default::default();
            }
            _ => {
                let (key, value) = field.split_at(
                    field
                        .find(':')
                        .ok_or(format!("field missing colon: {}", field))?
                        + 1,
                );
                match passport.update(key, value) {
                    Ok(v) => {
                        if v {
                            print!("\u{001b}[32m{}\u{001b}[0m ", field);
                        } else {
                            print!("\u{001b}[31m{}\u{001b}[0m ", field);
                        }
                    }
                    Err(_) => print!("\u{001b}[33m{}\u{001b}[0m ", field),
                }
            }
        }
    }
    Ok((count, total))
}

// i went a little overboard in anticipation of part 2 requiring us to do something with the values
#[derive(Default, Debug)]
struct Passport {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Passport {
    fn update(&mut self, field_name: &str, value: &str) -> Result<bool, String> {
        lazy_static! {
            static ref HEIGHT_IN: Regex = Regex::new(r"(^\d{2})in$").unwrap();
            static ref HEIGHT_CM: Regex = Regex::new(r"(^\d{3})cm$").unwrap();
            static ref HAIR_COLOR: Regex = Regex::new(r"^#[0-9a-z]{6}$").unwrap();
            static ref DIGIT_9: Regex = Regex::new(r"^\d{9}$").unwrap();
            static ref EYE_COLOR: HashSet<&'static str> = {
                let mut m: HashSet<&'static str> = HashSet::new();
                for col in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter() {
                    m.insert(col);
                }
                m
            };
        }
        match field_name {
            "byr:" => {
                self.byr = {
                    match value.parse() {
                        Ok(v) => value.len() == 4 && 1920 <= v && v <= 2002,
                        Err(_) => false,
                    }
                };
                Ok(self.byr)
            }
            "iyr:" => {
                self.iyr = {
                    match value.parse() {
                        Ok(v) => value.len() == 4 && 2010 <= v && v <= 2020,
                        Err(_) => false,
                    }
                };
                Ok(self.iyr)
            }
            "eyr:" => {
                self.eyr = {
                    match value.parse() {
                        Ok(v) => value.len() == 4 && 2020 <= v && v <= 2030,
                        Err(_) => false,
                    }
                };
                Ok(self.eyr)
            }
            "hgt:" => {
                self.hgt = {
                    if let Some(height) = HEIGHT_IN.captures(value) {
                        let v: u32 = height.get(1).unwrap().as_str().parse().unwrap();
                        59 <= v && v <= 76
                    } else if let Some(height) = HEIGHT_CM.captures(value) {
                        let v: u32 = height.get(1).unwrap().as_str().parse().unwrap();
                        150 <= v && v <= 193
                    } else {
                        false
                    }
                };
                Ok(self.hgt)
            }
            "hcl:" => {
                self.hcl = HAIR_COLOR.is_match(value);
                Ok(self.hcl)
            }
            "ecl:" => {
                self.ecl = EYE_COLOR.contains(&value);
                Ok(self.ecl)
            }
            "pid:" => {
                self.pid = DIGIT_9.is_match(value);
                Ok(self.pid)
            }
            "cid:" => Ok(true),
            _ => return Err(format!("Invalid field name: {}", field_name)),
        }
    }

    fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}
