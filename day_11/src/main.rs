#[macro_use] extern crate lazy_static;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::iter;

#[derive(Debug, Copy, Clone)]
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

    fn is_occupied(&self) -> bool {
        match self {
            Cell::Occupied => true,
            _ => false,
        }
    }

    fn is_seat(&self) -> bool {
        match self {
            Cell::Occupied => true,
            Cell::Free => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
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

    fn flip(&mut self, x: usize, y: usize) {
        assert!(x < self.width);
        assert!(y < self.height);

        let p = y * self.width + x;
        self.grid[p] =
            match self.grid[p] {
                Cell::Floor => Cell::Floor,
                Cell::Free => Cell::Occupied,
                Cell::Occupied => Cell::Free,
            }
    }

    fn is_occupied(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width as isize { return false; }
        if y < 0 || y >= self.height as isize { return false; }

        self.grid[y as usize * self.width + x as usize].is_occupied()
    }

    fn is_seat(&self, x: isize, y: isize) -> bool {
        if x < 0 || x >= self.width as isize { return false; }
        if y < 0 || y >= self.height as isize { return false; }

        self.grid[y as usize * self.width + x as usize].is_seat()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(isize, isize)> {
        let x = x as isize;
        let y = y as isize;
        vec![
            (x-1,y-1), (x-1,y), (x-1,y+1),
            (x,y-1), (x,y+1),
            (x+1,y-1), (x+1,y), (x+1,y+1),]
    }

    fn get_neighbor_2(&self,
                      x:usize,
                      y: usize,
                      dir: &(isize, isize)) -> Option<(isize, isize)> {

        let mut current_x = x as isize;
        let mut current_y = y as isize;
        loop {
            current_x += dir.0;
            current_y += dir.1;

            if current_x < 0 || current_x >= self.width as isize { return None; }
            if current_y < 0 || current_y >= self.height as isize { return None; }
            if self.is_seat(current_x, current_y) {
                return Some((current_x, current_y));
            }
        }
    }

    fn get_neighbors_2(&self, x: usize, y: usize) -> Vec<(isize, isize)> {
        lazy_static! {
            static ref directions: Vec<(isize, isize)> = vec![
                (-1,-1), (-1,0), (-1,1),
                (0,-1), (0,1),
                (1,-1), (1,0), (1,1),
            ];
        }

        let neighbors = 
        directions.iter()
            .flat_map(|dir|self.get_neighbor_2(x,y,&dir))
            .collect();

        neighbors
    }

    fn next_gen_2(&mut self) -> bool {
        let flips = (0..self.width).cycle()
            .zip((0..self.height).flat_map(|y| iter::repeat(y).take(self.width)))
            .filter(|&(x,y)| {
                match self.grid[y * self.width + x] {
                    Cell::Free => self.get_neighbors_2(x,y)
                        .iter()
                        .all(|(a,b)| !self.is_occupied(*a,*b)),
                    Cell::Occupied => self.get_neighbors_2(x,y)
                        .iter()
                        .filter(|(a,b)| self.is_occupied(*a,*b))
                        .count() >= 5,
                    Cell::Floor => false,
                }
            })
        .collect::<Vec<_>>();

        let ret = flips.len() > 0;
        for (x,y) in flips {
            self.flip(x,y);
        }

        ret
    }

    fn next_gen(&mut self) -> bool {
        let flips = (0..self.width).cycle()
            .zip((0..self.height).flat_map(|y| iter::repeat(y).take(self.width)))
            .filter(|&(x,y)| {
                match self.grid[y * self.width + x] {
                    Cell::Floor => false,
                    Cell::Free => self.get_neighbors(x,y)
                        .iter()
                        .all(|(a,b)| !self.is_occupied(*a,*b)),
                    Cell::Occupied => self.get_neighbors(x,y)
                        .iter()
                        .filter(|(a,b)| self.is_occupied(*a,*b))
                        .count() >= 4,
                }
            })
        .collect::<Vec<_>>();

        let ret = flips.len() > 0;
        for (x,y) in flips {
            self.flip(x,y);
        }

        ret
    }
}

fn read_input() -> Grid {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    Grid::parse(reader.lines().map(|x|x.unwrap()).collect())
}

fn part1(input: &Grid) {
    let mut grid = input.clone();
    loop {
        if grid.next_gen() == false { break; }
    }

    let occupied = grid.grid.iter()
        .filter(|c| c.is_occupied())
        .count();
    println!("Res: {}", occupied);
}

fn part2(input: &Grid) {
    let mut grid = input.clone();
    let mut i = 0;
    loop {
        if grid.next_gen_2() == false { break; }
        i += 1;
    }

    let occupied = grid.grid.iter()
        .filter(|c| c.is_occupied())
        .count();
    println!("Res: {}", occupied);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
