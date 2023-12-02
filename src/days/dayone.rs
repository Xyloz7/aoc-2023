use crate::common::lines_from_file;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn get_num_from_line(line: String) -> u32 {
    let mut digits = line.chars().filter(|x| x.is_numeric());
    let first_ = digits.next().unwrap();
    let last_ = digits.last().unwrap_or(first_);
    let str_num: String = vec![first_, last_].into_iter().collect();

    str_num.parse::<u32>().unwrap()
}

pub fn day_one() -> u32 {
    let lines = lines_from_file("./src/inputs/day1_input.txt");
    lines.into_iter().map(get_num_from_line).sum()
}

fn get_first(line_: String) -> u32 {
    let mut current_chars: Vec<char> = vec![];
    for character in line_.chars() {
        if character.is_numeric() {
            return character.to_digit(10).unwrap();
        }
        current_chars.push(character);
        let strnum: String = current_chars.clone().into_iter().collect();

        let digitnames = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for (digit, digitname) in digitnames.into_iter().enumerate() {
            if strnum.contains(digitname) {
                return digit as u32 + 1;
            }
        }
    }
    0
}

fn get_last(line_: String) -> u32 {
    let mut current_chars: Vec<char> = vec![];
    for character in line_.chars().rev() {
        if character.is_numeric() {
            return character.to_digit(10).unwrap();
        }
        current_chars.push(character);
        let strnum: String = current_chars.clone().into_iter().collect();
        let digitnames = [
            "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
        ];
        for (digit, digitname) in digitnames.into_iter().enumerate() {
            if strnum.contains(digitname) {
                return digit as u32 + 1;
            }
        }
    }
    0
}

fn combine_u32s(a: u32, b: u32) -> u32 {
    let combined_str = format!("{}{}", a, b);

    // Use parse to convert the combined string back to u32
    match combined_str.parse::<u32>() {
        Ok(result) => result,
        Err(_) => {
            // Handle parsing error (e.g., if the combined value is too large)
            panic!("Failed to parse combined u32 values");
        }
    }
}

pub fn day_one_part2() -> u32 {
    let lines = lines_from_file("./src/day1_input.txt");
    lines
        .into_iter()
        .map(|l| combine_u32s(get_first(l.clone()), get_last(l)))
        .sum()
}
