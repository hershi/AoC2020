#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

const WIDTH : usize = 10;
const HEIGHT : usize = 10;

#[derive(Debug)]
struct Tile {
    id: usize,
    grid: Vec<bool>,
}

type Input = Vec<Tile>;

impl Tile {
    fn parse(input: &[String]) -> Tile {
        lazy_static! {
            static ref TILE_ID_PARSER: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }

        let id = TILE_ID_PARSER.captures(&input[0]).unwrap()
            .get(1).unwrap()
            .as_str()
            .parse().unwrap();

        let grid = input[1..11].iter()
            .flat_map(|s| s.chars().map(|c| c == '#'))
            .collect();

        Tile{id, grid}
    }

    fn left_border(&self) -> Vec<bool> {
        self.grid.iter()
            .step_by(WIDTH)
            .cloned()
            .collect()
    }

    fn right_border(&self) -> Vec<bool> {
        self.grid.iter()
            .skip(WIDTH-1)
            .step_by(WIDTH)
            .cloned()
            .collect()
    }

    fn top_border(&self) -> Vec<bool> {
        self.grid[0..WIDTH].iter().cloned().collect()
    }

    fn bottom_border(&self) -> Vec<bool> {
        self.grid[self.grid.len()-WIDTH..].iter().cloned().collect()
    }
}

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let input = reader.
        lines()
        .map(|x|x.unwrap())
        .collect::<Vec<_>>();

    input.chunks(12)
        .map(|chunk| Tile::parse(chunk))
        .collect()
}

fn part1(input: &Input) {
    println!("Part 1: {:?}", input[0].left_border());
    println!("Part 1: {:?}", input[0].right_border());
    println!("Part 1: {:?}", input[0].top_border());
    println!("Part 1: {:?}", input[0].bottom_border());
}

fn part2(input: &Input) {
    //println!("Part 1: {:?}", input);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
