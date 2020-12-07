#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
struct Rule {
    color: String,
    contents: Vec<(String, usize)>,
}

impl Rule {
    fn parse(line: String) -> Rule {
        lazy_static! {
            static ref KEY: Regex = Regex::new(r"(.*) bags contain (.*)").unwrap();
            static ref CONTENTS: Regex = Regex::new(r"(\d+)\s*([^,]+) bags?,?(.*)").unwrap();
        }

        let c = KEY.captures(&line).unwrap();
        let key = c.get(1).unwrap().as_str().to_string();

        let mut s = c.get(2).unwrap().as_str();
        let mut contents = vec![];
        loop {
            let c = CONTENTS.captures(s);
            match c {
                None => break,
                Some(caps) => {
                    s = caps.get(3).unwrap().as_str();
                    let count = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                    let color = caps.get(2).unwrap().as_str().to_string();
                    contents.push((color, count));
                }
            }
        }

        Rule { color: key, contents }
    }
}

fn read_input() -> Vec<Rule> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| Rule::parse(l))
        .collect::<Vec<_>>()
}

fn part1(passports: &Vec<Rule>) {
    println!("Valid Part 1: {}", 1);
}

fn part2(passports: &Vec<Rule>) {
    println!("Valid Part 2: {}", 1);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    println!("input: {:?}", input);
    part1(&input);
    part2(&input);
}
