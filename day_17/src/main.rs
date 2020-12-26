#[macro_use] extern crate lazy_static;

use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::iter;

type Coordinates = (isize, isize, isize, isize);

#[derive(Debug, Clone)]
struct Grid {
    grid: HashSet<Coordinates>,
}

impl Grid {
    fn parse(input: Vec<String>) -> Grid {
        let height = input.len() as isize;
        let width = input[0].len() as isize;

        let grid = input.iter()
            .flat_map(|r| r.chars())
            .enumerate()
            .filter(|(i,c)| *c == '#')
            .map(|(i,_)| (i as isize % width, i as isize / height, 0, 0))
            .collect();

        Grid{grid}
    }

    fn flip(&mut self, c: &Coordinates) {
        //println!("Flipping {:?}", c);
        if !self.grid.remove(c) {
            self.grid.insert(*c);
        }
    }

    fn is_active(&self, c: &Coordinates) -> bool {
        self.grid.contains(c)
    }

    fn get_neighbors(cell: &Coordinates) -> Vec<Coordinates> {
        (0..(3isize.pow(4)))
            .map(|p| (p%3-1+cell.0, p/3%3-1+cell.1, p/9%3-1+cell.2, p/27%3-1+cell.3))
            .filter(|c| c != cell)
            .collect()
    }

    fn should_flip(&self, c: &Coordinates, active_neighbors: usize) -> bool {
        //println!("Should flip: {:?} {}", c, active_neighbors);
        (self.is_active(c) && active_neighbors != 3 && active_neighbors != 2) ||
            (!self.is_active(c) && active_neighbors == 3)

    }

    fn next_gen(&mut self) {
        println!("Next Gen...");
        let cells_to_visit : HashSet<Coordinates> = self.grid.iter()
            .flat_map(|c| Grid::get_neighbors(&c))
            .chain(self.grid.iter().cloned())
            .collect();

        let flips = cells_to_visit.iter()
            .filter(|c| {
                let active_neighbors = Grid::get_neighbors(&c).iter()
                .map(|n| self.is_active(n) as usize)
                .sum();

                self.should_flip(c, active_neighbors)
             })
            .collect::<Vec<_>>();

        for flip in flips {
            self.flip(flip);
        }
    }
}

fn read_input() -> Grid {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    Grid::parse(reader.lines().map(|x|x.unwrap()).collect())
}

fn part1(input: &Grid) {
    let mut input = input.clone();

    for _ in 0..6 {
        input.next_gen();
    }

    println!("Active cells = {}", input.grid.len());
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
}
