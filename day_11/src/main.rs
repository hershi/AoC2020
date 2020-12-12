#[macro_use] extern crate lazy_static;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
enum Cell {
    Floor,
    Free,
    Occupied,
}

impl Cell {
    fn parse(c: &char) -> Cell {
        match c {
            '#' => Cell::Occupied,
            'L' => Cell::Free,
            '.' => Cell::Floor,
            _ => panic!("Bad input"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn parse(input: Vec<String>) -> Grid {
        let height = input.len();
        let width = input[0].len();

        let grid = input.iter()
            .flat_map(|r| r.chars())
            .map(|c| Cell::parse(&c))
            .collect::<Vec<Cell>>();

        Grid{grid, width, height}
    }

    fn get(&self, x: usize, y: usize) -> &Cell {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);

        &self.grid[y * self.width + x]
    }

    fn flip(&mut self, x: usize, y: usize) {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);

        let p = y * self.width + x;
        self.grid[p] =
            match self.grid[p] {
                Cell::Floor => Cell::Floor,
                Cell::Free => Cell::Occupied,
                Cell::Occupied => Cell::Free,
            }
    }
}

fn read_input() -> Grid {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    Grid::parse(reader.lines().map(|x|x.unwrap()).collect())
}

fn part1(input: &Grid) {
    println!("{:?}", input);
}

fn part2(input: &Grid) {
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
