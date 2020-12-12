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
        .collect::<Vec<_>>()
}

fn split_entries(lines: Vec<String>) -> Vec<String> {
    lines.iter()
        .fold(vec!["".to_string()], |mut acc, l| {
            if l.trim().is_empty() {
                acc.push("".to_string());
            } else {
                let len = acc.len();
                acc[len-1] = acc[len-1].trim().to_string();
                acc[len-1].push(' ');
                acc[len-1].push_str(l);
            }
            acc
        })
}

fn parse_entries(entries: Vec<String>) -> Vec<HashMap<String, String>> {
    entries.iter()
        .map(|e| e.split(' ')
            .fold(HashMap::new(), |mut acc, p| {
                let kv = p.split(':').collect::<Vec<_>>();
                if kv.len() != 2 {
                    return acc;
                }

                acc.insert(kv[0].to_string(), kv[1].to_string());
                acc
            }))
        .collect::<Vec<HashMap<String, String>>>()
}

fn validate_usize(s: &str, low:usize, high:usize) -> bool {
    let x = s.parse::<usize>().unwrap();
    x >= low && x <= high
}

fn validate_height(s: &str) -> bool {
    let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    let caps = re.captures(s);
    if caps.is_none() { return false; }
    let caps = caps.unwrap();
    let num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let metric = caps.get(2).unwrap().as_str();

    if metric == "in" {
        num >= 59 && num <= 76
    } else {
        assert!(metric == "cm");
        num >= 150 && num <= 193
    }
}

fn validate_hair_color(s: &str) -> bool {
    let re = Regex::new(r"^\s*#[0-9a-f]{6}\s*$").unwrap();
    re.is_match(s)
}

fn validate_eye_color(s: &str) -> bool {
    let re = Regex::new(r"^\s*(amb|blu|brn|gry|grn|hzl|oth)\s*$").unwrap();
    re.is_match(s)
}

fn validate_pid(s: &str) -> bool {
    let re = Regex::new(r"^\s*\d{9}\s*$").unwrap();
    re.is_match(s)
}

fn part1(passports: &Vec<HashMap<String, String>>) {
    let valid = passports.iter()
        .filter(|passport| {
            passport.contains_key("byr") &&
            passport.contains_key("iyr") &&
            passport.contains_key("eyr") &&
            passport.contains_key("hgt") &&
            passport.contains_key("hcl") &&
            passport.contains_key("ecl") &&
            passport.contains_key("pid")

        })
        .count();

    println!("Valid Part 1: {}", valid);
}

fn part2(passports: &Vec<HashMap<String, String>>) {
    let have_fields = passports.iter()
        .filter(|passport| {
            passport.contains_key("byr") &&
            passport.contains_key("iyr") &&
            passport.contains_key("eyr") &&
            passport.contains_key("hgt") &&
            passport.contains_key("hcl") &&
            passport.contains_key("ecl") &&
            passport.contains_key("pid")

        }).collect::<Vec<_>>();

    let valid = have_fields.iter()
        .filter(|passport| {
            validate_usize(passport.get("byr").unwrap(), 1920, 2002) &&
            validate_usize(passport.get("iyr").unwrap(), 2010, 2020) &&
            validate_usize(passport.get("eyr").unwrap(), 2020, 2030) &&
            validate_height(passport.get("hgt").unwrap()) &&
            validate_hair_color(passport.get("hcl").unwrap()) &&
            validate_eye_color(passport.get("ecl").unwrap()) &&
            validate_pid(passport.get("pid").unwrap())
        })
        .count();


    println!("Valid Part 1: {}", valid);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    let entries = split_entries(input);
    let passports = parse_entries(entries);
    part1(&passports);
    part2(&passports);
}
