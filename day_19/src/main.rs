#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;


fn read_input(file: &str) -> Vec<String> {
    let input_file = File::open(file).unwrap();
    let reader = BufReader::new(input_file);
    reader.
        lines()
        .map(|x|x.unwrap())
        .collect()
}

#[derive(Debug, Clone)]
enum ParsedRule {
    SingleChar(char),
    RuleId(usize),
    Sequence(Vec<ParsedRule>),
    Or(Vec<ParsedRule>),
}

impl ParsedRule {
    fn parse(def: &str) -> ParsedRule {
        //println!("Parsing: {}", def);

        if def.starts_with('"') {
            //println!("Parsing as single char: {}", def);
            return ParsedRule::SingleChar(def.chars().nth(1).unwrap());
        }

        if def.contains("|") {
            //println!("Parsing as or: {}", def);
            return ParsedRule::Or(def.split("|")
                .map(|part| ParsedRule::parse(part))
                .collect::<Vec<_>>());
        }

        //println!("Parsing as sequence: {}", def);
        ParsedRule::Sequence(def.trim().split(" ")
            .map(|n| ParsedRule::RuleId(n.parse::<usize>().unwrap()))
            .collect())
    }

    fn is_final(&self) -> bool {
        match self {
            ParsedRule::SingleChar(_) => true,
            ParsedRule::Sequence(v) => v.iter().all(|r| r.is_final()),
            ParsedRule::Or(v) => v.iter().all(|r| r.is_final()),
            ParsedRule::RuleId(_) => false,
        }
    }

    fn expand_vec(v: &Vec<ParsedRule>, rules: &ParsedRules) -> (Vec<ParsedRule>, bool) {
        let processed = v.iter().map(|r| r.expand(rules)).collect::<Vec<_>>();
        let changed = processed.iter().any(|(_,changed)| *changed);
        let vec = processed.into_iter().map(|(r,_)|r).collect();
        (vec, changed)
    }

    fn expand(&self, rules: &ParsedRules) -> (ParsedRule, bool) {
        match self {
            ParsedRule::SingleChar(_) => (self.clone(), false),
            ParsedRule::Sequence(v) => {
                let (rules, changed) = ParsedRule::expand_vec(v, rules);
                (ParsedRule::Sequence(rules), changed)
            },
            ParsedRule::Or(v) => {
                let (rules, changed) = ParsedRule::expand_vec(v, rules);
                (ParsedRule::Or(rules), changed)
            },
            ParsedRule::RuleId(id) => {
                (rules.get(id).unwrap().clone(), true)
            },
        }
    }

    fn into_regex(&self) -> String {
        assert!(self.is_final());

        match self {
            ParsedRule::SingleChar(c) => format!("{}", c),
            ParsedRule::Sequence(v) => v.iter()
                .map(|r| vec!["(", &r.into_regex(), ")"].join(""))
                .collect(),
            ParsedRule::Or(v) =>{ v.iter()
                .map(|r| vec!["(", &r.into_regex(), ")"].join(""))
                .collect::<Vec<String>>()
                .join("|")
            },
            _ => panic!("Rule is not final..."),
        }
    }
}

type ParsedRules = HashMap<usize, ParsedRule>;

fn read_rules() -> ParsedRules {
    let rules = read_input("src/rules.txt");

    rules.iter()
        .map(|l| {
            lazy_static! {
                static ref RULE_PARSER: Regex = Regex::new(r"^(\d+):\s*(.*)\s*$").unwrap();
            }
            let cap = RULE_PARSER.captures(l).unwrap();
            let id = cap.get(1).unwrap().as_str().parse().unwrap();
            (id, ParsedRule::parse(cap.get(2).unwrap().as_str()))
        })
        .collect()
}

fn read_messages() -> Vec<String> {
    read_input("src/input.txt")
}

fn fully_expand_rule(rule: &ParsedRule, rules: &ParsedRules) -> String {
    println!("Expanding {:?}", rule);

    let mut rule = rule.clone();
    loop {
        let (r, changed) = rule.expand(rules);
        rule = r;
        if !changed { break; }
    }

    let regex = rule.into_regex();
    regex
}

fn part1(rules: &ParsedRules, messages: &Vec<String>) {
    let rule = rules.get(&0).unwrap();

    let rule = fully_expand_rule(rule, rules);

    //println!("Final rule ({}): {:?}", i, rule);
    let regex = vec!["^", &rule, "$"].join("");
    //println!("Final rule: {:?}", regex);

    let regex = Regex::new(&regex).unwrap();

    let valid = messages.iter()
        .filter(|m| regex.is_match(m))
        .count();

    println!("Valid count: {}", valid);
}

fn part2(rules: &ParsedRules, messages: &Vec<String>) {
    let rule = rules.get(&500).unwrap();

    let rule = fully_expand_rule(rule, rules);

    let regex = vec!["^(", &rule, ")$"].join("");
    //println!("Final rule: {:?}", regex);

    let regex = Regex::new(&regex).unwrap();

    let valid = messages.iter()
        .filter(|m| regex.is_match(m))
        .count();

    println!("Valid count: {}", valid);
}

fn main() {
    println!("Reading input");
    let rules = read_rules();
    let messages = read_messages();
    part1(&rules, &messages);
    part2(&rules, &messages);
}
