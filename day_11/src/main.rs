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

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        if x >= self.width { return false; }
        if y >= self.height { return false; }

        self.grid[y * self.width + x].is_occupied()
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let neighbors :Vec<(isize,isize)> = vec![
            (-1,-1), (-1,0), (-1,1),
            (0,-1), (0,1),
            (1,-1), (1,0), (1,1),];
        iter::repeat((x,y))
            .zip(neighbors.iter())
            .map(|((x,y), (dx,dy))| (x as isize + dx, y as isize+dy))
            .filter(|&(a,b)| a >= 0 && a < self.width as isize
                    && b >= 0 && b < self.height as isize)
            .map(|(x,y)| (x as usize, y as usize))
            .collect()
    }

    fn next_gen(&mut self) -> bool {
        let flips = (0..self.height)
            .flat_map(|y| iter::repeat(y).take(self.width))
            .zip((0..self.width).cycle())
            .map(|(y,x)|(x,y))
            .filter(|(x,y)| self.grid[y * self.width + x].is_seat())
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
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
