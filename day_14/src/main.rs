#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Clone)]
struct Mask {
    or_mask: u64,
    and_mask: u64,
    floating: Vec<usize>,
}

impl Mask {
    fn new(mask_str: &str) -> Mask {
        let mut or_mask = 0u64;
        let mut and_mask = u64::MAX;
        let mut floating = Vec::new();

        for (i, bit) in mask_str.chars().rev().enumerate() {
            match bit {
                '0' => and_mask &= !(1u64 << i),
                '1' => or_mask |= 1u64 << i,
                _ => floating.push(1usize << i),
            }
        }

        assert!(and_mask | or_mask == and_mask);
        assert!(and_mask & or_mask == or_mask);

        Mask {or_mask, and_mask, floating}
    }

    fn apply(&self, n: &u64) -> u64 {
        n & self.and_mask | self.or_mask
    }

    fn gen_addresses(&self, addr: &usize) -> Vec<usize> {
        let addr = addr | self.or_mask as usize;

        let mut result = Vec::new();
        for i in 0..(2usize.pow(self.floating.len() as u32)){
            let mut addr = addr;
            let mut j = 0;
            let mut i = i;
            while i > 0 {
                if i % 2 == 1 { addr ^= self.floating[j]; }
                i = i >> 1;
                j += 1;
            }

            result.push(addr);
        }

        result
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem(usize, u64),
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        lazy_static! {
            static ref MASK_PARSER: Regex = Regex::new(r"mask = ([01X]*)\s*$").unwrap();
            static ref MEM_PARSER: Regex = Regex::new(r"^mem\[(\d+)\]\s*=\s*(\d+)\s*$").unwrap();
        }

        if let Some(mask_parse) = MASK_PARSER.captures(line) {
            return Instruction::Mask(Mask::new(mask_parse.get(1).unwrap().as_str()));
        }

        if let Some(mem_parse) = MEM_PARSER.captures(line) {
            return Instruction::Mem(
                mem_parse.get(1).unwrap().as_str().parse().unwrap(),
                mem_parse.get(2).unwrap().as_str().parse().unwrap());
        }

        panic!(format!("Bad input {}", line));
    }
}

fn read_input() -> Vec<Instruction> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| Instruction::new(&l))
        .collect()
}

fn part1(input: &Vec<Instruction>) {
    let mut memory = HashMap::new();
    let mut mask = Mask::new("");
    for i in input {
        match i {
            Instruction::Mask(m)=> mask = m.clone(),
            Instruction::Mem(addr, val)=> { memory.insert(addr, mask.apply(val)); },
        }
    }

    println!("Part 1: {}", memory.values().sum::<u64>());
}

fn part2(input: &Vec<Instruction>) {
    let mut memory = HashMap::new();
    let mut mask = Mask::new("");
    for i in input {
        match i {
            Instruction::Mask(m)=> mask = m.clone(),
            Instruction::Mem(addr, val)=> {
                println!("{:?}: {}\n{:?}", mask, addr, mask.gen_addresses(addr));
                for a in mask.gen_addresses(addr) {
                    memory.insert(a, *val);
                }
            },
        }
    }

    println!("Part 2: {:?}", memory);
    println!("Part 2: {}", memory.values().sum::<u64>());
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
