use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

type Expression = String;

#[derive(Debug)]
enum Op {
    Plus,
    Mult,
}

impl Op {
    fn eval(&self, lhs: isize, rhs: isize) -> isize {
        match self {
            Op::Plus => lhs + rhs,
            Op::Mult => lhs * rhs,
        }
    }
}

fn read_input() -> Vec<Expression> {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.
        lines()
        .map(|x|x.unwrap())
        .map(|s| s.chars().filter(|c| !c.is_ascii_whitespace()).collect::<String>())
        .collect()
}

fn eval_expression(expr: &str) -> isize {
    //println!("eval_expression: {}", expr);
    let (mut lhs, mut remaining) = parse_num_or_parentheses(expr);

    while remaining.len() > 0 {
        //println!("eval_expression: {}\n\t{} | {}", expr, lhs, remaining);
        let (op, r) = parse_op(remaining);
        //println!("eval_expression: {}\n\t{} | {:?} | {}", expr, lhs, op, r);
        let (rhs, r) = parse_num_or_parentheses(r);

        lhs = op.eval(lhs, rhs);

        remaining = r;
    }

    println!("eval_expression {}: Returning {}", expr, lhs);
    lhs
}

fn parse_op(expr: &str) -> (Op, &str) {
    let o = expr.chars().nth(0).unwrap();

    let op = match o {
        '+' => Op::Plus,
        '*' => Op::Mult,
        _ => panic!("Bad Op {:?}", o),
    };

    (op, &expr[1..])
}

fn parse_num(expr: &str) -> (isize, &str) {
    let n = expr.chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();

    (n.trim().parse::<isize>().unwrap(), &expr[n.len()..])
}

fn parse_num_or_parentheses(expr: &str) -> (isize, &str) {
    if expr.chars().nth(0).unwrap().is_ascii_digit() {
        parse_num(expr)
    } else {
        assert!(expr.chars().nth(0).unwrap() == '(');
        parse_parentheses(expr)
    }


}

fn parse_parentheses(expr: &str) -> (isize, &str) {
    let closing_paren = find_closing_paren(expr);

    (eval_expression(&expr[1..=closing_paren]), &expr[closing_paren+2..])
}

fn find_closing_paren(expr: &str) -> usize {
    let mut count = 1;

    for (i, c) in expr.chars().skip(1).enumerate() {
        match c {
            ')' => count -= 1,
            '(' => count += 1,
            _ => (),
        }

        if count == 0 { return i; }
    }

    panic!("Malformed: {}", expr);
}

fn eval_expression_2(expr: &str) -> isize {
    if expr.chars().all(|c| c.is_ascii_digit()) {
        return expr.parse::<isize>().unwrap();
    }

    let mut expr = expr.to_string();
    loop {
        println!("In loop: {}", expr);
        let pos = expr.find('+');
        if pos.is_none() { break; }

        let pos = pos.unwrap();
        let (prefix, lhs) = find_lhs(&expr[0..pos]);
        let lhs = eval_expression_2(lhs);

        let (rhs, suffix) = find_rhs(&expr[pos+1..]);
        let rhs = eval_expression_2(rhs);

        println!("In loop 11: {} | {} | {}", prefix, lhs + rhs, suffix);
        let prefix = prefix.to_string();
        let suffix = suffix.to_string();

        expr = prefix;
        expr.push_str(&format!("{}", lhs + rhs));
        expr.push_str(&suffix);
    }

    println!("eval_expression_2: Calling eval_expression with {}", expr);
    eval_expression(&expr)
}

fn find_rhs(expr: &str) -> (&str, &str) {
    let n = expr.chars().take_while(|c| c.is_ascii_digit()).count();
    if n > 0 {
        return (&expr[0..n], &expr[n..]);
    }

    // RHS is not a number - it's a parentheses element
    let mut count = 0;

    for (i, c) in expr.chars().enumerate() {
        match c {
            ')' => count -= 1,
            '(' => count += 1,
            _ => (),
        }

        if count == 0 { return (&expr[0..=i], &expr[i+1..]); }
    }

    panic!("Malformed: {}", expr);
}


fn find_lhs(expr: &str) -> (&str, &str) {
    let n = expr.chars().rev().take_while(|c| c.is_ascii_digit()).count();
    if n > 0 {
        return (&expr[0..expr.len()-n], &expr[expr.len()-n..]);
    }

    // LHS is not a number - it's a parentheses element
    let mut count = 0;

    for (i, c) in expr.as_bytes().iter().enumerate().rev() {
        match *c as char {
            '(' => count -= 1,
            ')' => count += 1,
            _ => (),
        }

        if count == 0 { return (&expr[0..i], &expr[i..]); }
    }

    panic!("Malformed: {}", expr);
}

fn part1(input: &Vec<Expression>) {
    let res : isize = input.iter()
        .map(|e| eval_expression(e))
        .sum();
    println!("Part 1: {}", res);
}

fn part2(input: &Vec<Expression>) {
    let res : isize = input.iter()
        .map(|e| eval_expression_2(e))
        .sum();
    println!("Part 2: {}", res);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    //part1(&input);
    part2(&input);
}
