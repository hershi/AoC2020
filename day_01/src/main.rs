use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> HashSet<usize> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap().parse::<usize>().unwrap())
        .collect::<HashSet<_>>()
}

fn find_pair(input: &HashSet<usize>, sum: usize) -> Option<(usize, usize)> {
    let x1 = input.iter()
        .filter(|&&x| x < sum)
        .filter(|&x| input.contains(&(sum-x)))
        .last();

    x1.map(|x| (*x, sum-x))
}

fn part_1(input: &HashSet<usize>) {
    let (x1, x2) = find_pair(input, 2020).unwrap();
    println!("{} x {} == {}", x1, x2, x1 * x2);
}

fn part_2(input: &HashSet<usize>) {
    let (x1, (x2,x3)) = input.iter()
        .map(|x| (x, find_pair(input, 2020-x)))
        .filter(|(_, res)| res.is_some())
        .map(|(x, y)| (x, y.unwrap()))
        .take(1)
        .last()
        .unwrap();

    println!("{} x {} x {} == {}", x1, x2, x3, x1 * x2 * x3);
}

fn main() {
    println!("Raeding input");
    let input = read_input();
    println!("Input read");

    part_1(&input);
    part_2(&input);
}
