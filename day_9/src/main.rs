use std::collections::HashSet;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<usize> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.lines()
        .map(|x|x.unwrap())
        .map(|num|num.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn valid(window: &HashSet<usize>, num: &usize) -> bool {
    for n in window {
        if n >= num { continue; }
        if *num - n == *n { continue; }
        if window.contains(&(num-n)) { return true; }
    }

    return false;
}

fn part1(input: &Vec<usize>) -> usize {
    let window_size = 25;
    let mut sliding_window : HashSet<usize> = input.iter().take(window_size).cloned().collect();

    for i in window_size..input.len() {
        let n = input[i];
        if !valid(&sliding_window, &n) {
            println!("Part 1: Invalid: {}", n);
            return n;
        }

        sliding_window.remove(&input[i-window_size]);
        sliding_window.insert(n);
    }

    println!("Couldn't find a number!");
    return 0;
}

fn part2(input: &Vec<usize>, n: usize) {
    let mut sum = input[0];
    let mut start_index = 0;
    let mut end_index = 0;

    loop {
        if sum == n && end_index > start_index {
            let mut x = input.iter()
                .skip(start_index)
                .take(end_index-start_index+1)
                .collect::<Vec<_>>();

            x.sort();

            println!("{}-{}: {}+{} == {}",
                     start_index,
                     end_index,
                     x[0],
                     x[x.len()-1],
                     x[0] + x[x.len()-1]);


            return;
        }

        while sum > n && start_index < end_index{
            sum -= input[start_index];
            start_index += 1;
        }

        if end_index >= input.len() { break; }

        while sum < n {
            end_index += 1;
            sum += input[end_index];
        }
    }

    println!("Part 2: Couldn't find it!");
}

fn main() {
    println!("Reading input");
    let input = read_input();
    let n = part1(&input);
    part2(&input, n);
}
