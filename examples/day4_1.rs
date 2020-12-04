use std::fs::File;
use std::io::Read;

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
                }
                passport = Default::default();
            }
            _ => {
                let (key, _) = field.split_at(field.find(':').ok_or(format!("field missing colon: {}", field))?);
                passport.update(key, true)?;
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
    cid: bool
}

impl Passport {
    fn update(&mut self, field_name: &str, value: bool) -> Result<(), String> {
        match field_name {
            "byr" => self.byr = value,
            "iyr" => self.iyr = value,
            "eyr" => self.eyr = value,
            "hgt" => self.hgt = value,
            "hcl" => self.hcl = value,
            "ecl" => self.ecl = value,
            "pid" => self.pid = value,
            "cid" => self.cid = value,
            _ => return Err(format!("Invalid field name: {}", field_name)),
        }
        Ok(())
    }
    
    fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
}