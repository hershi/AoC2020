use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<(usize, usize)> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l|map_entry(l.split_at(7)))
        .collect::<Vec<_>>()
}

fn map_entry((r,c): (&str, &str)) -> (usize, usize) {
    (map_row(r), map_col(c))
}

fn map_row(r: &str) -> usize {
    r.chars()
        .map(|c| if c == 'F' {0} else {1} )
        .fold(0, |acc, b| { acc * 2 + b })
}

fn map_col(c: &str) -> usize {
    c.chars()
        .map(|c| if c == 'L' {0} else {1} )
        .fold(0, |acc, b| { acc * 2 + b })
}

fn calc_seat_id((r,c): &(usize, usize)) -> usize {
    r * 8 + c
}


fn part_1(input: &Vec<(usize, usize)>) {
    let res = input.iter()
        .map(|e| calc_seat_id(e))
        .max()
        .unwrap();
    println!("Part 1: {}", res);
}

fn part_2(input: &Vec<(usize, usize)>) {
    let mut seat_ids = input.iter()
        .map(|e| calc_seat_id(e))
        .collect::<Vec<_>>();

    seat_ids.sort();

    println!("IDs: {:?}", seat_ids);
    for idx in 1..seat_ids.len() {
        if seat_ids[idx-1] + 2 == seat_ids[idx] {
            println!("Res: {}", seat_ids[idx] - 1);
        }
    }
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part_1(&input);
    part_2(&input);
}
