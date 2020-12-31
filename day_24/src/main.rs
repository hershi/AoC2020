use std::collections::HashMap;
use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    East,
    West,
    Southeast,
    Southwest,
    Northeast,
    Northwest,
}

impl Direction {
    fn parse(s: &str) -> Option<Direction> {
        match s {
            "e"=> Some(Direction::East),
            "w"=> Some(Direction::West),
            "se"=> Some(Direction::Southeast),
            "sw"=> Some(Direction::Southwest),
            "ne"=> Some(Direction::Northeast),
            "nw"=> Some(Direction::Northwest),
            _ => None,
        }
    }

    // Translate to canonical form which is (north, east)
    fn to_canonical(&self) -> (isize, isize){
        match self {
            Direction::East => (0,2),
            Direction::West => (0,-2),
            Direction::Southeast => (-1,1),
            Direction::Southwest => (-1,-1),
            Direction::Northeast => (1,1),
            Direction::Northwest => (1,-1),
        }
    }
}

type Instruction = HashMap<Direction, usize>;

type Input = Vec<(isize, isize)>;

type TileSet = HashSet<(isize,isize)>;

fn parse_input_line(line: &str) -> (isize, isize) {
    let mut pos = 0;
    let mut result = HashMap::new();

    while pos < line.len() {
        if let Some(d) = Direction::parse(&line[pos..pos+1]) {
            *result.entry(d).or_insert(0) += 1;
            pos += 1;
            continue;
        }

        if let Some(d) = Direction::parse(&line[pos..pos+2]) {
            *result.entry(d).or_insert(0) += 1;
            pos += 2;
            continue;
        }

        panic!("Should never get here");
    }

    canonical(result)
}

fn canonical(inst: Instruction) -> (isize, isize) {
    inst.iter()
        .map(|(d, c)| (d.to_canonical(), *c as isize))
        .map(|((n, e), c)| (n*c, e*c))
        .fold((0,0), |acc, (n,e)| (acc.0+n, acc.1+e))
}

fn read_input(file: &str) -> Input {
    let input_file = File::open(file).unwrap();
    let reader = BufReader::new(input_file);
    reader.
        lines()
        .map(|x|parse_input_line(&x.unwrap()))
        .collect()
}

fn flip(board: &mut TileSet, loc: &(isize, isize)) {
    if !board.insert(loc.clone()) {
        board.remove(&loc);
    }
}

fn init_board(input: &Input) -> TileSet {
    let mut result = HashSet::new();
    for i in input {
        flip(&mut result, i);
    }

    result
}

fn part_1(input: &Input) {
    let init_board = init_board(input);
    println!("Part 1: {:#?}", init_board.len());
}

fn get_neighbors(loc: &(isize,isize)) -> Vec<(isize, isize)> {
    vec![
            Direction::East.to_canonical(),
            Direction::West.to_canonical(),
            Direction::Southeast.to_canonical(),
            Direction::Southwest.to_canonical(),
            Direction::Northeast.to_canonical(),
            Direction::Northwest.to_canonical(),
    ].into_iter()
        .map(|(n,e)| (loc.0 + n, loc.1 + e))
        .collect()
}

fn should_flip(loc: &(isize, isize), board: &TileSet) -> bool {
    let black_neighbors = get_neighbors(loc).iter()
        .filter(|x| board.contains(x))
        .count();

    if board.contains(loc) {
        // Tile is black. Flip it if it has 0 or more than 2 neighbors
        // that are black
        black_neighbors == 0 || black_neighbors > 2
    } else {
        // Tile is white. Flip it if it has exatly 2 black neighbors
        black_neighbors == 2
    }
}

fn next_day(mut board: TileSet) -> TileSet {
    let to_check = board.iter()
        .flat_map(|loc| get_neighbors(loc))
        .chain(board.iter().cloned())
        .collect::<TileSet>();

    let to_flip = to_check.into_iter()
        .filter(|loc| should_flip(loc, &board))
        .collect::<Vec<_>>();

    for i in to_flip {
        flip(&mut board, &i);
    }

    board
}

fn part_2(input: &Input) {
    let mut board = init_board(input);
    //for i in 0..100 {
    for i in 0..100 {
        println!("Day {}: {}", i, board.len());
        board = next_day(board);
    }

    println!("Part 2: {}", board.len());
}

fn main() {
    println!("Reading input");
    let input = read_input("src/input.txt");
    part_1(&input);
    part_2(&input);
}
