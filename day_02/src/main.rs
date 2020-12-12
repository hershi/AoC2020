use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<String> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .collect::<Vec<String>>()
}

struct ParsedLine {
    c: char,
    low: usize,
    high: usize,
    password: String,
}

fn parse_line(line: &str) -> ParsedLine {
    let re = Regex::new(r"^(\d+)-(\d+)\s+(.):\s*(.*)$").unwrap();
    let caps = re.captures(line).unwrap();
    ParsedLine {
        c: caps.get(3).unwrap().as_str().chars().last().unwrap(),
        low: caps.get(1).unwrap().as_str().parse().unwrap(),
        high: caps.get(2).unwrap().as_str().parse().unwrap(),
        password: caps.get(4).unwrap().as_str().to_string()}
}

fn is_valid_part1(line: &str) -> bool {
    let parsed_line = parse_line(line);
    let char_map = parsed_line.password
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

    let count = *(char_map.get(&parsed_line.c).unwrap_or(&0));
    count >= parsed_line.low && count <= parsed_line.high
}

fn is_valid_part2(line: &str) -> bool {
    let parsed_line = parse_line(line);
    let c1 = parsed_line.password.chars().nth(parsed_line.low-1).map_or(false, |c| c==parsed_line.c);
    let c2 = parsed_line.password.chars().nth(parsed_line.high-1).map_or(false, |c| c==parsed_line.c);

    c1 ^ c2
}

fn part_1(input: &Vec<String>) {
    let valid_count = input.iter()
        .filter(|x| is_valid_part1(x))
        .count();

    println!("Valid passwords count: {}", valid_count);
}

fn part_2(input: &Vec<String>) {
    let valid_count = input.iter()
        .filter(|x| is_valid_part2(x))
        .count();

    println!("Valid passwords count: {}", valid_count);
}

fn main() {
    println!("Raeding input");
    let input = read_input();
    println!("Input read");

    part_1(&input);
    part_2(&input);
}
