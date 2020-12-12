#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
enum Opcode {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Opcode {
    fn new(line: &str) -> Opcode {
        lazy_static! {
            static ref PARSER: Regex = Regex::new(r"^(...)\s*([+-]\d*)\s*$").unwrap();
        }

        let cap = PARSER.captures(line).unwrap();
        match cap.get(1).unwrap().as_str() {
            "acc" => Opcode::Acc(cap.get(2).unwrap().as_str().parse().unwrap()),
            "jmp" => Opcode::Jmp(cap.get(2).unwrap().as_str().parse().unwrap()),
            _ => Opcode::Nop(cap.get(2).unwrap().as_str().parse().unwrap()),
        }
    }
}

fn read_input() -> Vec<Opcode> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| Opcode::new(&l))
        .collect()
}

fn part1(opcodes: &Vec<Opcode>) -> Result<isize, &str> {
    let mut acc = 0;
    let mut i : isize = 0;
    let mut visited = HashSet::new();

    loop {
        if i == opcodes.len() as isize {
            return Ok(acc);
        }

        if i > opcodes.len() as isize || i < 0 {
            return Err("out of range");
        }

        if visited.contains(&i) {
            return Err("looped");
        }

        visited.insert(i);
        match opcodes[i as usize] {
            Opcode::Acc(x) => { acc += x; i+= 1; },
            Opcode::Jmp(x) => { i += x; },
            Opcode::Nop(_) => { i += 1;} ,
        }
    }
}

fn flip_opcode(op: &Opcode) -> Opcode {
    match op {
        Opcode::Acc(x) => { Opcode::Acc(*x) }
        Opcode::Jmp(x) => { Opcode::Nop(*x) }
        Opcode::Nop(x) => { Opcode::Jmp(*x) } ,
    }
}

fn part2(opcodes: &mut Vec<Opcode>) {
    for i in 0..opcodes.len() {
        if let Opcode::Acc(_) = opcodes[i] { continue; }
        opcodes[i] = flip_opcode(&opcodes[i]);
        let res = part1(&opcodes);
        if res.is_ok() { println!("{}: acc {}", i, res.unwrap()); }
        opcodes[i] = flip_opcode(&opcodes[i]);
    }
}

fn main() {
    println!("Reading input");
    let mut opcodes = read_input();
    part1(&opcodes);
    part2(&mut opcodes);
}
