use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<Vec<String>> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .fold(vec![vec![]], |mut acc, l| {
            if l.trim().is_empty() {
                acc.push(Vec::new());
            } else {
                let mut x = acc.pop().unwrap();
                x.push(l);
                acc.push(x);
            }

            acc
        })
}

fn process_group(group: &Vec<String>) -> HashMap<char, usize> {
    group.join("")
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            (*acc.entry(c).or_insert(0)) += 1;
            acc
        })
}

fn part1(input: &Vec<Vec<String>>) {
    let result:usize = input.iter()
        .map(|entry| process_group(entry).len())
        .sum();
    println!("Valid Part 1: {:?}", result);
}

fn part2(input: &Vec<Vec<String>>) {
    let result:usize = input.iter()
        .map(|entry| process_group(entry).iter()
                .filter(|(_, v)| **v == entry.len())
                .count())
        .sum();
    println!("Valid Part 2: {}", result);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
