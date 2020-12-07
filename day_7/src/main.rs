#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn parse_rule(line: String) -> (String, Vec<(String, usize)>) {
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

    (key, contents)
}

fn invert_rules(rules: &HashMap<String, Vec<(String, usize)>>) -> HashMap<String, String> {
    rules.iter()
        .flat_map(|rule| rule.1.iter()
                  .map(|c| (c.0.clone(), rule.0.clone()))
                  .collect::<Vec<_>>())
        .collect()
}

fn read_input() -> HashMap<String, Vec<(String, usize)>> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| parse_rule(l))
        .collect()
}

fn part1(rules: &HashMap<String, Vec<(String, usize)>>,
         inverted: &HashMap<String, String>) {
    println!("Valid Part 1: {:?}", inverted);
}

fn part2(rules: &HashMap<String, Vec<(String, usize)>>,
         inverted: &HashMap<String, String>) {
    println!("Valid Part 2: {}", 1);
}

fn main() {
    println!("Reading input");
    let rules = read_input();
    let inverted = invert_rules(&rules);
    part1(&rules, &inverted);
    part2(&rules, &inverted);
}
