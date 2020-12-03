use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Clear,
    Tree
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(data: Vec<Vec<Cell>>) -> Map {
        let height = data.len();
        let width = data[0].len();
        Map {data, width, height}
    }

    fn get(&self, x: usize, y: usize) -> &Cell {
        let real_x = x % self.width;
        &self.data[y][real_x]
    }
}

fn read_input() -> Map {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let input = reader.lines()
        .map(|x|x.unwrap())
        .map(|l| l.chars()
                   .map(|c| if c == '.' { Cell::Clear } else { Cell::Tree })
                   .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Map::new(input)
}

fn check_slope(map: &Map, slope: (usize, usize)) -> usize {
    let mut pos = (0,0);

    let mut trees = 0;
    while pos.0 < map.height {
        if *map.get(pos.1, pos.0) == Cell::Tree { trees += 1; }

        pos.0 += slope.0;
        pos.1 += slope.1;
    }

    trees
}

fn part1(map: &Map) {
    let trees = check_slope(map, (1,3));
    println!("Trees: {}", trees);
}

fn part2(map: &Map) {

    let t1 = check_slope(map, (1,1));
    let t2 = check_slope(map, (1,3));
    let t3 = check_slope(map, (1,5));
    let t4 = check_slope(map, (1,7));
    let t5 = check_slope(map, (2,1));

    println!("{}, {}, {}, {}, {}", t1, t2, t3, t4, t5);
    println!("{}", t1 * t2 * t3 * t4 * t5);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
