use std::fs::File;
use std::io::prelude::*;
extern crate regex;
use regex::Regex;

pub fn solve() {
    let instructions = parse();
    solve_both(&instructions);
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Instruction {
    operation: String,
    a: i64,
    b: i64,
    c: i64,
}

fn parse() -> Vec<Instruction> {
    let filename = "input/day21input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let instruction_re = Regex::new(r"([A-Za-z]+) (\d+) (\d+) (\d+)").unwrap();

    instruction_re
        .captures_iter(&contents)
        .map(|cap| Instruction {
            operation: cap[1].to_string(),
            a: cap[2].parse::<i64>().unwrap(),
            b: cap[3].parse::<i64>().unwrap(),
            c: cap[4].parse::<i64>().unwrap(),
        })
        .collect()
}

fn solve_both(instructions: &Vec<Instruction>) {
    let pointer_register = 4;

    let mut results = Vec::new();

    let mut instructions_executed = 0;

    let mut registers: Vec<i64> = vec![0, 0, 0, 0, 0, 0];
    let mut instruction_pointer = 0;

    while instructions.get(instruction_pointer as usize).is_some() {
        let instruction = &instructions[instruction_pointer as usize];

        if instruction_pointer == 28 {
            results.push((registers[1], instructions_executed));
        }

        registers[pointer_register] = instruction_pointer;

        str_to_fun(&instruction.operation)(
            &mut registers,
            instruction.a,
            instruction.b,
            instruction.c,
        );

        instruction_pointer = registers[pointer_register] + 1;

        instructions_executed += 1;

        if instructions_executed >= std::i32::MAX {
            break;
        }
    }

    println!("{:?}", results);

    let (_, smallest) = results.iter().min_by_key(|(_, v)| v).unwrap();
    let (part_one, _) = results
        .iter()
        .filter(|(_, v)| v == smallest)
        .min_by_key(|(z, _)| z)
        .unwrap();

    println!("Part one: {}", part_one);

    let (_, smallest) = results.iter().max_by_key(|(_, v)| v).unwrap();
    let (part_two, _) = results
        .iter()
        .filter(|(_, v)| v == smallest)
        .min_by_key(|(z, _)| z)
        .unwrap();

    println!("Part two: {}", part_two);
}

fn addr(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] + registers[b as usize];
}

fn addi(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] + b;
}

fn mulr(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] * registers[b as usize];
}

fn muli(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] * b;
}

fn banr(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] & registers[b as usize];
}

fn bani(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] & b;
}

fn borr(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] | registers[b as usize];
}

fn bori(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    registers[c as usize] = registers[a as usize] | b;
}

fn setr(registers: &mut Vec<i64>, a: i64, _b: i64, c: i64) {
    registers[c as usize] = registers[a as usize];
}

fn seti(registers: &mut Vec<i64>, a: i64, _b: i64, c: i64) {
    registers[c as usize] = a;
}

fn gtir(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    if a > registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn gtri(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    if registers[a as usize] > b {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn gtrr(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    if registers[a as usize] > registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn eqir(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    if a == registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn eqri(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    if registers[a as usize] == b {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn eqrr(registers: &mut Vec<i64>, a: i64, b: i64, c: i64) {
    if registers[a as usize] == registers[b as usize] {
        registers[c as usize] = 1;
    } else {
        registers[c as usize] = 0;
    }
}

fn str_to_fun(s: &String) -> fn(&mut Vec<i64>, i64, i64, i64) {
    match s.as_ref() {
        "addr" => addr,
        "addi" => addi,
        "mulr" => mulr,
        "muli" => muli,
        "banr" => banr,
        "bani" => bani,
        "borr" => borr,
        "bori" => bori,
        "setr" => setr,
        "seti" => seti,
        "gtir" => gtir,
        "gtri" => gtri,
        "gtrr" => gtrr,
        "eqir" => eqir,
        "eqri" => eqri,
        "eqrr" => eqrr,
        _ => addr,
    }
}
