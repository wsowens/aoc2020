use std::convert::TryFrom;
use std::fs::File;
use std::io::{self, BufRead};

enum Instr {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

enum Terminate {
    InfiniteLoop(i32),
    Complete(i32),
    BadAddress(i32),
}

fn main() {
    let mut code = parse_program("inputs/day8.txt").unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(-1);
    });

    // part 1
    if let Terminate::InfiniteLoop(value) = run_program(&code) {
        println!("Part 1 accumulator value: {}", value);
    }

    // part 2
    for addr in 0..code.len() {
        let mut fixed = match &code[addr] {
            &Instr::Nop(v) => Instr::Jmp(v),
            &Instr::Jmp(v) => Instr::Nop(v),
            _ => continue,
        };
        std::mem::swap(&mut code[addr], &mut fixed);
        if let Terminate::Complete(value) = run_program(&code) {
            println!("Part 2 accumulator value: {}", value)
        }
        std::mem::swap(&mut code[addr], &mut fixed);
    }
}

fn parse_program(filename: &str) -> Result<Vec<Instr>, String> {
    let file = File::open("inputs/day8.txt").map_err(|e| format!("{}", e))?;

    let mut program: Vec<Instr> = vec![];

    for line in io::BufReader::new(file).lines() {
        let line = line.map_err(|e| format!("{}", e))?;
        let mut parts = line.split_ascii_whitespace();
        let instr = parts.next().ok_or(format!("No instruction"))?;
        let value = parts
            .next()
            .ok_or(format!("No value"))?
            .parse()
            .map_err(|e| format!("{}", e))?;
        program.push(match instr {
            "nop" => Instr::Nop(value),
            "jmp" => Instr::Jmp(value),
            "acc" => Instr::Acc(value),
            _ => return Err(format!("Bad instruction: {}", instr)),
        })
    }
    Ok(program)
}

fn run_program(code: &[Instr]) -> Terminate {
    let mut visited = vec![false; code.len()];
    let mut addr = 0;
    let mut acc = 0;
    loop {
        let address = match usize::try_from(addr) {
            Ok(a) => a,
            Err(_) => {
                break Terminate::BadAddress(addr);
            }
        };
        if (address) == code.len() {
            break Terminate::Complete(acc);
        }
        if visited[address] {
            break Terminate::InfiniteLoop(acc);
        }
        visited[address] = true;
        match code[address] {
            Instr::Nop(_) => addr += 1,
            Instr::Jmp(value) => addr += value,
            Instr::Acc(value) => {
                addr += 1;
                acc += value;
            }
        }
    }
}
