use std::io::prelude::*;

type Val = usize;
type Input = Vec<Val>;

#[derive(Debug, Clone)]
struct Item {
    next: usize,
    prev: usize,
}

#[derive(Debug, Clone)]
struct Cups {
    items: Vec<Item>,
    current: usize,
    size: usize,
}

impl Cups {
    fn new(input: Input) -> Cups {
        let mut items = std::iter::repeat(Item{next: 0, prev:0})
            .take(input.len())
            .collect::<Vec<_>>();

        for i in 0..input.len() {
            let next = (i+1) % input.len();
            items[input[i]-1].next = input[next]-1;
        }

        for i in 0..items.len() {
            let next = items[i].next;
            items[next].prev = i;
        }

        Cups {
            items,
            current: input[0]-1,
            size: input.len(),
        }
    }

    fn remove(&mut self, count: usize) -> Vec<Val> {
        let mut new_next = self.items[self.current].next;

        let mut res = Vec::with_capacity(count);
        for _ in 0..count {
            res.push(new_next + 1);
            new_next = self.items[new_next].next;
        }

        self.items[self.current].next = new_next;
        self.items[new_next].prev = self.current;

        self.size -= count;

        res
    }

    fn insert(&mut self, pos: Val, vals: Vec<Val>) {
        let mut prev_val = pos - 1;
        let count = vals.len();

        for v in vals {
            let v = v-1;
            let next = self.items[prev_val].next;
            self.items[v].next = next;
            self.items[v].prev = prev_val;
            self.items[prev_val].next = v;
            self.items[next].prev = v;

            prev_val = v;
        }

        self.size += count;
    }

    fn advance(&mut self, offset: usize) {
        for _ in 0..offset {
            self.current = self.items[self.current].next;
        }
    }

    fn get_current(&self) -> Val {
        self.current + 1
    }

    fn print(&self) {
        let mut current = self.current;

        for _ in 0..self.size {
            print!("{} ", current + 1);
            current = self.items[current].next;
        }

        println!("");
    }
}

fn read_input() -> Input {
    let mut input = 318946572usize;
    let mut result = Vec::new();
    while input > 0 {
        result.push((input % 10) as usize);
        input /= 10;
    }

    result.into_iter().rev().collect()
}

fn read_input_2() -> Input {
    let mut input = 318946572usize;
    let mut result = Vec::new();
    while input > 0 {
        result.push((input % 10) as usize);
        input /= 10;
    }

    let max = *result.iter().max().unwrap();

    result.into_iter().rev()
        .chain(max+1..=1000000).collect()
}

fn find_next(current: usize, min: usize, max :usize, excluded: &Vec<usize>) -> usize {
    let mut new_val = current;
    loop {
        if new_val == min { new_val = max; } else { new_val -= 1; }

        if excluded.iter().any(|&v| v == new_val) { continue; }
        return new_val;
    }
}

fn process_2(mut cups: Cups, num_moves: usize) -> Cups {
    let min = 1;
    let max = cups.size;
    let mut current_cup = cups.get_current();

    for i in 0..num_moves {
        let to_move = cups.remove(3);
        let destination = find_next(current_cup, min, max, &to_move);
        cups.insert(destination, to_move);

        cups.advance(1);
        current_cup = cups.get_current();

        if i % 100000 == 0 {
            println!("{}: new current_cup is {}", i, current_cup);
        }
    }

    cups
}

fn process(input: &Input, num_moves: usize) -> Input{
    let size = input.len();
    let min = input.iter().min().unwrap();
    let max = input.iter().max().unwrap();
    let mut current_cup = input[0];


    let mut cups = input.clone();
    println!("{} : {:?}", current_cup, cups);
    for i in 0..num_moves {
        let to_move = cups.iter()
            .cycle()
            .skip_while(|v| **v != current_cup)
            .skip(1)
            .take(3)
            .cloned()
            .collect::<Vec<_>>();

        let next_val = find_next(current_cup, *min, *max, &to_move);

        cups = cups.iter()
            .filter(|v| to_move.iter().all(|x| x != *v))
            .cloned()
            .collect();

        cups = cups.iter()
            .take_while(|v| **v != next_val).cloned()
            .chain(std::iter::once(next_val))
            .chain(to_move.iter().cloned())
            .chain(cups.iter().skip_while(|v| **v != next_val).skip(1).cloned())
            .collect();

        current_cup = *cups.iter()
            .cycle()
            .skip_while(|v| **v != current_cup)
            .skip(1)
            .nth(0)
            .unwrap();

        println!("{} : {:?}", current_cup, cups);
    }

    println!("result: {:?}", cups);
    cups
}

fn part_1() {
    let cups = process(&read_input(), 100);

    let cups = cups.iter()
        .cycle()
        .skip_while(|v| **v != 1)
        .skip(1)
        .take(cups.len()-1)
        .map(|n| format!("{}", n)).collect::<Vec<_>>();
    println!("Result {}", cups.as_slice().join(""));
    println!("Part 1: {:?}", cups);
}

fn part_2() {
    let mut cups = Cups::new(read_input_2());
    let mut cups = process_2(cups, 10_000_000);

    while cups.get_current() != 1 {
        cups.advance(1);
    }

    cups.advance(1);
    let v1 = cups.get_current();
    cups.advance(1);
    let v2 = cups.get_current();

    println!("{} * {} = {}", v1, v2, v1 * v2);
}

fn main() {
    part_1();
    part_2();
}
