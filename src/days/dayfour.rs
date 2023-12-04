use std::collections::{HashMap, HashSet};

use log::debug;

use super::common::lines_from_file;

fn get_number_sets(line: String, skip_n: usize) -> (HashSet<u32>, HashSet<u32>) {
    let lotto_numbers: String = line.chars().skip(skip_n).collect();
    let mut number_sets = lotto_numbers.split(" | ");
    let winning_numbers: HashSet<u32> = number_sets
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let scratchcard_numbers: HashSet<u32> = number_sets
        .next()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    (winning_numbers, scratchcard_numbers)
}

fn card_score(winning: &HashSet<u32>, actual: &HashSet<u32>) -> u32 {
    let score = actual
        .iter()
        .fold(0, |acc, &no| acc + winning.contains(&no) as u32);

    if score > 0 {
        2_u32.pow(score - 1)
    } else {
        0
    }
}

pub fn part1() -> u32 {
    let lines = lines_from_file("./src/inputs/day4.txt");
    let mut total_score = 0;
    let skip_n = lines.first().unwrap().split(":").next().unwrap().len() + 1;
    for line in lines {
        let (winning_numbers, scratchcard_numbers) = get_number_sets(line, skip_n);
        total_score += card_score(&winning_numbers, &scratchcard_numbers);
    }
    total_score
}

fn card_score2(winning: &HashSet<u32>, actual: &HashSet<u32>) -> u32 {
    actual
        .iter()
        .fold(0, |acc, &no| acc + winning.contains(&no) as u32)
}

fn create_map(n: u32) -> HashMap<u32, u32> {
    let mut map = HashMap::new();
    for i in 0..=n-1 {
        map.insert(i, 1);
    }
    map
}

pub fn part2() -> u32 {
    let lines = lines_from_file("./src/inputs/day4.txt");
    let mut card_count = create_map(lines.len() as u32);
    let total_cards = lines.len() as u32;
    let skip_n = lines.first().unwrap().split(":").next().unwrap().len() + 1;
    for (card_id, line) in lines.into_iter().enumerate() {
        let (winning_numbers, scratchcard_numbers) = get_number_sets(line, skip_n);
        let cards_won = card_score2(&winning_numbers, &scratchcard_numbers);
        for _ in 0..card_count[&(card_id as u32)] {
            for i in 1..=cards_won {
                let key = (card_id as u32 + i).min(total_cards);
                card_count.entry(key).and_modify(|x| *x += 1);
            }
        }

        debug!(
            "{} of Card {} wins cards {}",
            card_count[&(card_id as u32)],
            card_id,
            card_score2(&winning_numbers, &scratchcard_numbers)
        );
    }
    card_count.values().sum()
}
