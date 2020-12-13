use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> (usize, Vec<(usize, usize)>) {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let input = reader.lines()
        .map(|x|x.unwrap())
        .collect::<Vec<_>>();

    (input[0].parse::<usize>().unwrap(),
    input[1].split(",")
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, n)| (i, n.parse().unwrap()))
        .collect())
}

fn part1(timestamp: usize, lines: &Vec<usize>) {
    println!("Part 1: {}",
             lines.iter()
             .map(|l| (l, l - (timestamp % l)))
             .min_by_key(|(_,b)| *b)
             .map(|(a,b)| a * b)
             .unwrap());

}

fn valid_chain(t: usize, lines: &Vec<(usize, usize)>) -> usize {
    let c = lines.iter()
        .skip(1)
        .map(|(i, n)| if ((t + i) % n) == 0 { Some(n) } else {None} )
        .take_while(|x| x.is_some())
        .count();

    c
}

fn part2(lines: &Vec<(usize, usize)>) {
    let mut search :Option<(usize, usize)> = None;
    let mut steps = lines[0].1;
    let mut current_chain = 0;
    let mut t = 0;

    loop {
        let c = valid_chain(t, lines);
        if c == lines.len() - 1 {
            println!("Part 2: timestamp {}", t);
            return;
        }

        if search.is_some() && c >= search.unwrap().0 {
            println!("{}: Increasing steps from {} to {}", t, steps, t - search.unwrap().1);
            steps = t - search.unwrap().1;
            current_chain = search.unwrap().0;
            search = None;
        }

        if c > current_chain {
            search = Some((c, t));
        }

        t += steps;

    }
}

fn main() {
    println!("Reading input");
    let (timestamp, lines) = read_input();

    println!("{:?}",  lines);
    part1(timestamp, &lines.iter().map(|(_, n)| *n).collect());
    part2(&lines);
}
