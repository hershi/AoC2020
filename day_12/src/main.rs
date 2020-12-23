#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(usize),
    R(usize),
    F(isize),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    E,
    W,
    N,
    S,
}

impl Instruction {
    fn new(line: &str) -> Instruction {
        lazy_static! {
            static ref PARSER: Regex = Regex::new(r"^(.)(\d+)\s*$").unwrap();
        }

        let cap = PARSER.captures(line).unwrap();
        match cap.get(1).unwrap().as_str() {
            "N" => Instruction::N(cap.get(2).unwrap().as_str().parse().unwrap()),
            "S" => Instruction::S(cap.get(2).unwrap().as_str().parse().unwrap()),
            "E" => Instruction::E(cap.get(2).unwrap().as_str().parse().unwrap()),
            "W" => Instruction::W(cap.get(2).unwrap().as_str().parse().unwrap()),
            "L" => Instruction::L(cap.get(2).unwrap().as_str().parse().unwrap()),
            "R" => Instruction::R(cap.get(2).unwrap().as_str().parse().unwrap()),
            "F" => Instruction::F(cap.get(2).unwrap().as_str().parse().unwrap()),
            _ => panic!("Bad input"),
        }
    }
}

fn read_input() -> Vec<Instruction> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|l| Instruction::new(&l))
        .collect()
}

fn make_move(dir: &Direction, dist: &isize, loc: (isize, isize)) -> (isize, isize) {
    match dir {
        Direction::N => (loc.0 + dist, loc.1),
        Direction::S => (loc.0 - dist, loc.1),
        Direction::E =>  (loc.0, loc.1 + dist),
        Direction::W => (loc.0, loc.1 - dist),
    }
}

fn turn(direction: &Direction, angle: &usize) -> Direction {
    let helper = vec![Direction::N, Direction::E, Direction::S, Direction::W];

    let current = match direction {
        Direction::N => 0,
        Direction::E => 1,
        Direction::S => 2,
        Direction::W => 3,
    };

    helper[(current + angle/90) % helper.len()].clone()
}

fn part1(input: &Vec<Instruction>) {
    let mut direction = Direction::E;
    let mut loc = (0,0);

    for i in input {
        match i {
            Instruction::N(x) => loc = make_move(&Direction::N, x, loc),
            Instruction::S(x) => loc = make_move(&Direction::S, x, loc),
            Instruction::E(x) => loc = make_move(&Direction::E, x, loc),
            Instruction::W(x) => loc = make_move(&Direction::W, x, loc),
            Instruction::L(x) => direction = turn(&direction, &(360 - x)),
            Instruction::R(x) => direction = turn(&direction, x),
            Instruction::F(x) => loc = make_move(&direction, x, loc),
        }
    }

    println!("Part 1: {:?}", loc);
    println!("{}", loc.0.abs() + loc.1.abs());
}

type Waypoint = (isize, isize);

fn rotate_waypoint(waypoint: Waypoint, angle: &usize) -> Waypoint {
    match angle {
        90 => (-waypoint.1, waypoint.0),
        180 => (-waypoint.0, -waypoint.1),
        270 => (waypoint.1, -waypoint.0),
        _ => panic!("Illegal angle"),
    }
}

fn part2(input: &Vec<Instruction>) {
    let mut waypoint = (1, 10);
    let mut loc = (0,0);

    for i in input {
        match i {
            Instruction::N(x) => waypoint = make_move(&Direction::N, x, waypoint),
            Instruction::S(x) => waypoint = make_move(&Direction::S, x, waypoint),
            Instruction::E(x) => waypoint = make_move(&Direction::E, x, waypoint),
            Instruction::W(x) => waypoint = make_move(&Direction::W, x, waypoint),
            Instruction::L(x) => waypoint = rotate_waypoint(waypoint, &(360 - x)),
            Instruction::R(x) => waypoint = rotate_waypoint(waypoint, x),
            Instruction::F(x) => {
                loc.0 += waypoint.0 * x;
                loc.1 += waypoint.1 * x;
            },
        }
    }

    println!("Part 2: {:?}", loc);
    println!("{}", loc.0.abs() + loc.1.abs());
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
