#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::cmp::min;
use std::cmp::max;

lazy_static! {
    pub static ref MY_TICKET : Vec<usize> =
        vec![97,61,53,101,131,163,79,103,67,127,71,109,89,107,83,73,113,59,137,139];

    pub static ref RULES : HashMap<&'static str, (usize, usize, usize, usize)> =
        [("departure location", (27, 180, 187, 953)),
        ("departure station", (47, 527, 545, 958)),
        ("departure platform", (36, 566, 572, 973)),
        ("departure track", (37, 497, 505, 971)),
        ("departure date", (47, 707, 719, 969)),
        ("departure time", (36, 275, 290, 949)),
        ("arrival location", (31, 855, 864, 955)),
        ("arrival station", (50, 148, 158, 949)),
        ("arrival platform", (50, 441, 467, 965)),
        ("arrival track", (30, 648, 659, 962)),
        ("class", (26, 470, 481, 966)),
        ("duration", (27, 808, 818, 958)),
        ("price", (49, 769, 784, 970)),
        ("route", (49, 796, 809, 964)),
        ("row", (42, 362, 383, 971)),
        ("seat", (34, 877, 887, 952)),
        ("train", (31, 354, 363, 950)),
        ("type", (39, 208, 231, 953)),
        ("wagon", (47, 736, 746, 968)),
        ("zone", (44, 290, 310, 974))].iter().cloned().collect();
}

fn read_input() -> Vec<Vec<usize>> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l|l.split(",").map(|n| n.parse().unwrap()).collect::<Vec<usize>>())
        .collect::<Vec<_>>()
}

fn is_valid(val: usize, valid_ranges: &Vec<(usize, usize)>) -> bool {
    valid_ranges.iter()
        .any(|&(l,h)| val >= l && val <= h)
}

fn get_valid_ranges() -> Vec<(usize, usize)> {
    let mut valid_ranges : Vec<(usize, usize)> = RULES
        .values()
        .flat_map(
            |rule| vec![(rule.0, rule.1), (rule.2, rule.3)])
        .collect();

    valid_ranges.sort_by_key(|e| e.0);

    let len = valid_ranges.len();
    for i in 0..valid_ranges.len()-1 {
        let i = len - i - 1;
        if valid_ranges[i-1].1 >= valid_ranges[i].0 {
            valid_ranges[i-1].1 = max(valid_ranges[i].1, valid_ranges[i-1].1);
            valid_ranges.remove(i);
        }
    }

    valid_ranges
}

fn part1(input: &Vec<Vec<usize>>) {
    let valid_ranges = get_valid_ranges();

    let error_scanning_rate = input.iter()
        .flat_map(|ticket| ticket.iter().filter(|f| !is_valid(**f, &valid_ranges)))
        .sum::<usize>();

    println!("Part 1: {:?}", error_scanning_rate);
}

fn part2(input: &Vec<Vec<usize>>) {
    let valid_ranges = get_valid_ranges();

    let valid_tickets = input.iter()
        .filter(|ticket| ticket.iter().all(|f| is_valid(*f, &valid_ranges)))
        .collect::<Vec<_>>();

    let mut ranges = valid_tickets[0].iter()
        .map(|&val| (val, val))
        .collect::<Vec<(usize, usize)>>();


    for ticket in input {
        for (i, &val) in ticket.iter().enumerate() {
            ranges[i].0 = min(ranges[i].0, val);
            ranges[i].1 = max(ranges[i].1, val);
        }
    }

    println!("Part 2: {:?}", ranges);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
