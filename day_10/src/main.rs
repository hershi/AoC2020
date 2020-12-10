#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<usize> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|n|n.parse().unwrap())
        .collect()
}

fn find_device_joltage(input: &Vec<usize>) -> usize {
    *input.iter().max().unwrap() + 3
}

fn find_chain_counts(current_joltage: usize,
                   target_joltage: usize,
                   lookup: &mut HashMap<usize, Option<usize>>) -> usize {
    //println! ("----- Find {} {}", current_joltage, target_joltage);
    if target_joltage == current_joltage {
        return 1;
    }

    let mut from_here = 0;

    for x in current_joltage+1..=current_joltage+3 {
        if !lookup.contains_key(&x) {
            continue;
        }

        if let Some(v) = lookup.get(&x).unwrap() {
            from_here += v;
            continue;
        }

        let tmp = find_chain_counts(x, target_joltage, lookup);
        lookup.insert(x, Some(tmp));
        from_here += tmp;
    }

    println!("{}: {}", current_joltage, from_here);
    from_here
}

fn part1(input: &Vec<usize>) {
    let device_joltage = find_device_joltage(input);
    let mut chain = input.clone();
    chain.sort();
    chain.push(device_joltage);
    println!("chain {:?}", chain);

    let mut prev_joltage = 0;
    let mut skips = vec![0,0,0];
    for x in chain {
        skips[x - prev_joltage - 1] += 1;
        prev_joltage = x;
    }

    println!("{:?} {}", skips, skips[0] * skips[2]);
}

fn part2(input: &Vec<usize>) {
    let device_joltage = find_device_joltage(input);

    let mut lookup : HashMap<usize, Option<usize>> =
        input.iter()
            .chain(vec![device_joltage].iter())
            .map(|j|(*j, None))
            .collect();

    let res =find_chain_counts(0, device_joltage, &mut lookup);

    println!("{:?}", lookup);
    println!("{}", res);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
