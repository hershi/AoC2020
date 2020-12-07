#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
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

fn invert_rules(rules: &HashMap<String, Vec<(String, usize)>>) -> HashMap<String, Vec<String>> {
    rules.iter()
        .flat_map(|rule| rule.1.iter()
                  .map(|c| (c.0.clone(), rule.0.clone()))
                  .collect::<Vec<_>>())
        .fold(HashMap::new(), |mut acc, (c1, c2)| {
            (*acc.entry(c1).or_insert(Vec::new())).push(c2);
            acc
        })
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
         inverted: &HashMap<String, Vec<String>>) {
    let mut to_check = inverted.get("shiny gold").unwrap().clone();
    let mut result = HashSet::new();
    while !to_check.is_empty() {
        let color = to_check.pop().unwrap();
        if inverted.contains_key(&color) {
            for c in inverted.get(&color).unwrap() {
                if !result.contains(c) {
                    to_check.push(c.to_string());
                }
            }
        }

        result.insert(color);
    }

    println!("part 1: {:?}", result);
    println!("part 1: {}", result.len());
}

fn find_num(color: &str,
            rules: &HashMap<String, Vec<(String, usize)>>,
            lookup: &mut HashMap<String, usize>) -> usize {
    if lookup.contains_key(color) {
        return *lookup.get(color).unwrap();
    }

    rules.get(color).unwrap().iter()
        .map(|(c, n)| {
            let num = find_num(c, rules, lookup);
            lookup.insert(c.clone(), num);
            println!("aaa: {} {} {}", c, n, num);
            (num + 1) * n
        })
        .sum()
}

fn part2(rules: &HashMap<String, Vec<(String, usize)>>) {
    let mut lookup = HashMap::new();
    let n = find_num("shiny gold", rules, &mut lookup);
    println!("Valid Part 2: {}", n);
}

fn main() {
    println!("Reading input");
    let rules = read_input();
    let inverted = invert_rules(&rules);
    part1(&rules, &inverted);
    part2(&rules);
}
