use std::collections::{hash_map::RandomState, HashSet};

use itertools::Itertools;

#[macro_use]
mod help;

fn main() {
    let data = input_to_str_iterator!("4");

    let games = data
        .enumerate()
        .map(|game| {
            let mut sides = game.1.split(":").last().unwrap().split("|");
            let winners: HashSet<usize, RandomState> = HashSet::from_iter(
                sides
                    .next()
                    .unwrap()
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap()),
            );
            let possibles = Vec::from_iter(
                sides
                    .next()
                    .unwrap()
                    .split(" ")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<usize>().unwrap()),
            );
            (game.0 + 1, winners, possibles)
        })
        .collect::<Vec<_>>();

    let count = games
        .iter()
        .map(|(_, winners, possibles)| possibles.iter().filter(|p| winners.contains(p)).count())
        .collect_vec();

    part1!(count
        .iter()
        .map(|count| {
            if *count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum::<usize>());

    let mut actual_counts = vec![1; games.len()];
    for i in 0..actual_counts.len() {
        let c = actual_counts[i];
        for x in (i + 1)..(i + 1 + count[i]) {
            if let Some(value) = actual_counts.get_mut(x) {
                *value += c;
            }
        }
    }

    part2!(actual_counts.iter().sum::<usize>());
}
