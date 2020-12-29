#[macro_use] extern crate lazy_static;

use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
struct Entry {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Entry {
    fn parse(line: &str) -> Entry {
        lazy_static! {
            static ref ENTRY_PARSER : Regex = Regex::new(r"^(.*)\s\((.*)\)$").unwrap();
        }

        let captures = ENTRY_PARSER.captures(line).unwrap();
        let ingredients = captures.get(1).unwrap().as_str()
            .split_whitespace()
            .map(|i| i.trim().to_string())
            .collect();

        let allergens = captures.get(2).unwrap().as_str()
            .split(&[' ', ','][..])
            .skip(1)
            .filter(|a| !a.trim().is_empty())
            .map(|a| a.trim().to_string())
            .collect();

        Entry { ingredients, allergens }
    }
}

type Input = Vec<Entry>;

fn read_input() -> Input {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);
    reader.
        lines()
        .map(|x|x.unwrap())
        .map(|x|Entry::parse(&x))
        .collect::<Vec<_>>()
}

fn get_all_ingredients(input: &Input) -> HashSet<String> {
    input.iter()
        .flat_map(|e| e.ingredients.iter())
        .cloned()
        .collect()
}

fn get_all_allergens(input: &Input) -> HashSet<String> {
    input.iter()
        .flat_map(|e| e.allergens.iter())
        .cloned()
        .collect()
}

fn intersect(v: &Vec<HashSet<String>>) -> HashSet<String> {
    v.iter()
        .skip(1)
        .fold(v[0].clone(), |acc, hs| acc.intersection(hs).cloned().collect())
}

fn part1(input: &Input) {
    let mut ingredients_to_allergens : HashMap<String, HashSet<String>> = HashMap::new();

    for entry in input {
        for ingredient in entry.ingredients.iter() {
            let i = ingredients_to_allergens.entry(ingredient.clone()).or_insert(HashSet::new());
            *i = i.union(&entry.allergens).cloned().collect();
        }
    }

    let mut allergens_to_ingredients = HashMap::new();
    for entry in input {
        for allergen in entry.allergens.iter() {
            allergens_to_ingredients
                .entry(allergen.clone())
                .or_insert(Vec::new())
                .push(entry.ingredients.clone());
        }
    }

    let mut allergens_to_ingredients = allergens_to_ingredients.into_iter()
        .map(|(a, v)| (a, intersect(&v)))
        .collect::<HashMap<String, HashSet<String>>>();

    let mut mapping = HashMap::new();
    loop {
        let selection = allergens_to_ingredients.iter()
            .filter(|(_,ingredients)| ingredients.len() == 1)
            .map(|(allergen,ingredients)| (allergen.clone(), ingredients.iter().last().unwrap().clone()))
            .collect::<Vec<(String, String)>>();

        if selection.len() == 0 { break; }

        for (allergen,ingredient) in selection {
            for entry in allergens_to_ingredients.iter_mut() {
                entry.1.remove(&ingredient);
            }

            let res = mapping.insert(allergen, ingredient);
            assert!(res.is_none());
        }
    }

    let all_ingredients = get_all_ingredients(input);
    let ingredients_with_allergens = mapping.values().cloned().collect::<HashSet<_>>();
    let ingredients_without_allergens =
        all_ingredients.difference(&ingredients_with_allergens).collect::<HashSet<_>>();

    let answer = input.iter()
        .map(|e| e.ingredients.iter().filter(|i| ingredients_without_allergens.contains(*i)).count())
        .sum::<usize>();

    println!("Ingredients without allergens appear {} times", answer);

    let mut mapping = mapping.iter()
        .collect::<Vec<_>>();

    mapping.sort_by_key(|x| x.0);
    println!("Mapping: {:#?}", mapping);

    let list = mapping.iter()
        .map(|x|x.1.as_str())
        .collect::<Vec<&str>>()
        .join(",");

    println!("List: {}", list);
}

fn main() {
    println!("Reading input");
    let input = read_input();
    part1(&input);
}
