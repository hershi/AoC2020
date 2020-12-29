use std::cmp;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

type Hand = VecDeque<usize>;
type Input = (Hand, Hand);

fn read_hand(filename: &str) -> Hand {
    let input_file = File::open(filename).unwrap();
    let reader = BufReader::new(input_file);
    reader.
        lines()
        .map(|x|x.unwrap().parse().unwrap())
        .collect()
}

fn read_input() -> Input {
    (read_hand("src/player_1.txt"), read_hand("src/player_2.txt"))
}

fn calc_score(hand: &Hand) -> usize {
    hand.iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i+1)*v)
        .sum::<usize>()
}

fn part_1(mut player_1: Hand, mut player_2: Hand) {
    while !player_1.is_empty() && !player_2.is_empty() {
        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        let winner = if p1 > p2 { &mut player_1 } else { &mut player_2 };

        //println!("{} {} {:#?}", p1, p2, winner);
        winner.push_back(cmp::max(p1,p2));
        winner.push_back(cmp::min(p1,p2));
    }

    let winner = if player_1.is_empty() { &player_2 } else { &player_1};

    let result = calc_score(winner);

    println!("Part 1: {}", result);
}

// true - player 1 won
// false - player 2 won
// usize - the score of the winner
fn play_recursive(mut player_1: Hand, mut player_2: Hand) -> (bool, usize) {
    let mut history = HashSet::new();

    loop {
        //println!("Player 1: {:?}\nPlayer 2: {:?}", player_1, player_2);
        // Handle game-end due to one player has all cards
        if player_1.is_empty() { return (false, calc_score(&player_2)); }
        if player_2.is_empty() { return (true, calc_score(&player_1)); }

        // Handle game-end due to infinite-recursion protection
        let mut hasher = DefaultHasher::new();
        (&player_1, &player_2).hash(&mut hasher);
        let hash = hasher.finish();

        if history.contains(&hash) {
            println!("Player 1 wins due to infinite recursion protection");
            return (true, calc_score(&player_1));
        }

        history.insert(hash);

        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        // Can we recurse?
        if p1 <= player_1.len() && p2 <= player_2.len() {
            // Yes!
            println!("Recurse!");
            let rec_hand_1 = player_1.iter().take(p1).cloned().collect();
            let rec_hand_2 = player_2.iter().take(p2).cloned().collect();
            let (player_1_won,_) = play_recursive(rec_hand_1, rec_hand_2);

            let (c1, c2) = if player_1_won { (p1,p2) } else { (p2,p1) };

            let winner = if player_1_won { &mut player_1 } else { &mut player_2 };
            winner.push_back(c1);
            winner.push_back(c2);
            continue;
        }

        // Can't recurse
        //println!("Don't Recurse!");
        let winner = if p1 > p2 { &mut player_1 } else { &mut player_2 };
        winner.push_back(cmp::max(p1,p2));
        winner.push_back(cmp::min(p1,p2));
    }
}

fn part_2(mut player_1: Hand, mut player_2: Hand) {
    println!("{:#?}", play_recursive(player_1, player_2));
}


fn main() {
    let (player_1, player_2) = read_input();
    part_1(player_1.clone(), player_2.clone());
    //part_2(vec![9, 2, 6, 3, 1].into_iter().collect(), vec![5, 8, 4, 7, 10].into_iter().collect());
    part_2(player_1, player_2);
}
