#[macro_use] extern crate lazy_static;

use std::fmt;
use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

lazy_static! {
    static ref ROTATIONS : Vec<Rotation> = vec![Rotation::Deg0, Rotation::Deg90, Rotation::Deg180, Rotation::Deg270];
}

type Grid = Vec<bool>;

#[derive(Clone)]
struct Tile {
    grid: Grid,
    width: usize,
    height: usize,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(f)
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(f)
    }
}

impl Tile {
    fn parse(input: &[String], width: usize, height: usize) -> (usize, Tile) {
        lazy_static! {
            static ref TILE_ID_PARSER: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        }

        let id = TILE_ID_PARSER.captures(&input[0]).unwrap()
            .get(1).unwrap()
            .as_str()
            .parse().unwrap();

        let grid = input[1..1+height].iter()
            .flat_map(|s| s.chars().map(|c| c == '#'))
            .collect();

        (id, Tile{grid, width: width, height: height})
    }

    fn left_border(&self) -> Grid {
        self.grid.iter()
            .step_by(self.width)
            .cloned()
            .collect()
    }

    fn right_border(&self) -> Grid {
        self.grid.iter()
            .skip(self.width-1)
            .step_by(self.width)
            .cloned()
            .collect()
    }

    fn top_border(&self) -> Grid {
        self.grid[0..self.width].iter().cloned().collect()
    }

    fn bottom_border(&self) -> Grid {
        self.grid[self.grid.len()-self.width..].iter().cloned().collect()
    }

    fn flip_vertical(&self) -> Tile {
        Tile { grid: self.grid.chunks(self.width)
            .flat_map(|chunk| chunk.iter().rev().cloned())
                .collect(),
            width: self.width,
            height: self.height}
    }

    fn flip_horizontal(&self) -> Tile {
        Tile { grid: self.grid.chunks(self.width).rev()
            .flat_map(|chunk| chunk.iter().cloned())
                .collect(),
            width: self.width,
            height: self.height}
    }

    fn rotate_90(&self) -> Tile {
        let mut result = vec![false;self.grid.len()];

        for (i,b) in self.grid.iter().enumerate() {
            let x = i % self.width;
            let y = i / self.width;
            let (x,y) = (self.width-y-1,x);
            let j = y*self.width + x;
            result[j] = *b;
        }

        Tile { grid: result,
            width: self.height,
            height: self.width}
    }

    fn rotate(&self, rotation: &Rotation) -> Tile {
        match rotation {
            Rotation::Deg0 => self.clone(),
            Rotation::Deg90 => self.rotate_90(),
            Rotation::Deg180 => self.flip_horizontal().flip_vertical(),
            Rotation::Deg270 => self.rotate(&Rotation::Deg180).rotate_90(),
        }
    }

    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n{}",
                 self.grid.iter()
                 .map(|&b| if b { '#' } else { '.' })
                 .collect::<Vec<_>>()
                 .chunks(self.width)
                 .map(|chunk| chunk.iter().collect::<String>())
                 .map(|l| format!("{}\n", l))
                 .collect::<String>())
    }

    fn get_oriented_tiles(&self) -> Vec<OrientedTile> {
        std::iter::repeat(false).take(ROTATIONS.len())
            .chain(std::iter::repeat(true).take(ROTATIONS.len()))
            .zip(ROTATIONS.iter().cycle())
            .map(|(flip, rotation)| {
                let t = if flip { self.flip_horizontal() } else { self.clone() };
                (flip, rotation, t.rotate(rotation)) })
            .map(|(flipped, &rotation, tile)| OrientedTile{tile, flipped, rotation})
            .collect()
    }
}

#[derive(Debug, Clone)]
struct OrientedTile {
    tile: Tile,
    flipped: bool,
    rotation: Rotation,
}

#[derive(Debug, Clone)]
struct Board {
    board: Vec<(usize, OrientedTile)>,
    tiles_per_edge: usize,
}

impl Board {
    fn can_place(&self, tile: &Tile) -> bool {
        let pos = self.board.len();

        if pos >= self.tiles_per_edge {
            let top_tile = &self.board[pos-self.tiles_per_edge].1.tile;
            if top_tile.bottom_border() != tile.top_border() {
                return false;
            }
        }

        if pos % self.tiles_per_edge > 0 {
            let left_tile = &self.board[pos - 1].1.tile;
            if left_tile.right_border() != tile.left_border() {
                return false;
            }
        }

        return true;
    }

    fn print(&self) -> String {
        let mut res = String::new();
        if self.board.len() == 0 { return res; }

        let width = self.board[0].1.tile.width;
        let height = self.board[0].1.tile.height;

        for row in self.board.chunks(self.tiles_per_edge) {
            for y in 0.. height {
                for t in row {
                    for x in 0..width {
                        res.push(if t.1.tile.grid[y * width + x] { '#' } else { '.' });
                    }
                    res.push('|');
                }
                res.push('\n');
            }
            res.push_str("---------------------------------------------------\n");
        }

        res
    }

    fn stitch(&self) -> Tile {
        assert!(self.board.len() > 0);
        let mut res = Vec::new();

        let width = self.board[0].1.tile.width;
        let height = self.board[0].1.tile.height;

        for row in self.board.chunks(self.tiles_per_edge) {
            for y in 1..height-1 {
                for t in row {
                    for x in 1..width-1 {
                        res.push(t.1.tile.grid[y * width + x]);
                    }
                }
            }
        }

        let tpe = self.tiles_per_edge;
        Tile { grid: res, width: (width-2)*tpe, height: (height-2)*tpe }
    }
}

type Input = HashMap<usize, Tile>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let input = reader.
        lines()
        .map(|x|x.unwrap())
        .collect::<Vec<_>>();

    input.chunks(12)
        .map(|chunk| Tile::parse(chunk, 10, 10))
        .collect()
}

fn solve(board: Board, remaining_tiles: &Input) -> Option<Board> {
    if remaining_tiles.len() == 0 {
        return Some(board);
    }

    for (id, tile) in remaining_tiles.iter() {
        for t in tile.get_oriented_tiles()
            .into_iter()
                .filter(|oriented_tile| board.can_place(&oriented_tile.tile)) {
                    let mut remaining = remaining_tiles.clone();
                    remaining.remove(id);

                    let mut board = board.clone();
                    board.board.push((*id, t));

                    if let Some(solution) = solve(board, &remaining) {
                        return Some(solution);
                    }
                }
    }

    None
}

fn create_sea_monster() -> Tile {
    let input = vec![ 
        "Tile 0000:".to_string(),
        "..................#.".to_string(),
        "#....##....##....###".to_string(),
        ".#..#..#..#..#..#...".to_string(),];

    Tile::parse(input.as_slice(), 20, 3).1
}

fn rows_match(monster: &[bool], row: &[bool]) -> bool {
    assert!(monster.len() == row.len());

    monster.iter().zip(row.iter())
        .filter(|(m,_)| **m)
        .all(|(m,r)| m == r)
}

fn find_monsters(tile: &Tile) {
    let monster = create_sea_monster();

    let mw = monster.width;
    let m1 = &monster.grid[0..mw];
    let m2 = &monster.grid[mw..mw*2];
    let m3 = &monster.grid[mw*2..];

    let mut locations = Vec::new();
    for x in 0..tile.width - monster.width {
        for y in 0..tile.height - monster.height {
            let w = tile.width;
            let r1 = &tile.grid[(y*w+x)..(y*w+x+mw)];
            let r2 = &tile.grid[((y+1)*w+x)..((y+1)*w+x+mw)];
            let r3 = &tile.grid[((y+2)*w+x)..((y+2)*w+x+mw)];

            if rows_match(m1, r1) && rows_match(m2, r2) && rows_match(m3, r3) {
                locations.push((x,y));
            }
        }
    }

    if locations.len() > 0 {
        let x = tile.grid.iter().filter(|b| **b).count();
        let monster_x = monster.grid.iter().filter(|b|**b).count();

        println!("Part 2: {} - ({} * {}) == {}", x, locations.len(), monster_x, x - (locations.len()*monster_x));
    }
}

fn part1(input: &Input) {
    let tiles_per_edge = (input.len() as f32).sqrt() as usize;
    let solution = solve(Board { board: Vec::new() , tiles_per_edge}, input);

    println!("Solution? {:?}", solution);
    if let Some(b) = solution {
        println!("{}", b.print());

        let top_left = 0;
        let top_right = b.tiles_per_edge-1;
        let bottom_left = b.tiles_per_edge * (b.tiles_per_edge - 1);
        let bottom_right = b.tiles_per_edge * b.tiles_per_edge - 1;

        let top_left = b.board[top_left].0;
        let top_right = b.board[top_right].0;
        let bottom_left = b.board[bottom_left].0;
        let bottom_right = b.board[bottom_right].0;

        println!("{} {} {} {} : {}", top_left, top_right, bottom_left, bottom_right,
                 top_left* top_right* bottom_left* bottom_right);


        let tile = b.stitch();
        println!("{}", tile);

        for rotation in ROTATIONS.iter() {
            println!("Finding monsters {:?}", rotation);
            find_monsters(&tile.rotate(rotation));
        }
        for rotation in ROTATIONS.iter() {
            println!("Finding monsters {:?} (flipped)", rotation);
            find_monsters(&tile.flip_horizontal().rotate(rotation));
        }
    }
}

fn part2(_: &Input) {
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
    part2(&input);
}
