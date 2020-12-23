#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<usize> {
     vec![1,12,0,20,8,16]
}

fn part1(input: &Vec<usize>) {
    process(input, 2020);
}

fn process(input: &Vec<usize>, turn: usize) {
    let mut history = input.iter()
        .enumerate()
        .map(|(i,v)| (*v,i))
        .take(input.len() - 1)
        .collect::<HashMap<usize, usize>>();

    let mut last_num = *input.last().unwrap();
    let mut current_turn = input.len();

    while current_turn < turn {
        let num = history.get(&last_num).map_or(0, |v| current_turn - 1 - v);
        if current_turn % 100000 == 0 || current_turn == (turn - 1) {
            println!("Turn {}: {}", current_turn + 1, num);
        }

        history.insert(last_num, current_turn - 1);
        last_num = num;
        current_turn += 1;
    }
}

fn part2(input: &Vec<usize>) {
    process(input, 30000000);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
