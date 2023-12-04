use std::collections::HashSet;

use super::common::lines_from_file;

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

        total_score += card_score(&winning_numbers, &scratchcard_numbers);
    }
    total_score
}

pub fn part2() {}
