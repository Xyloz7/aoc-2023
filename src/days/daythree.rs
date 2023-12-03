use std::collections::{HashMap, HashSet};

use log::debug;

use crate::common::lines_from_file;

fn vector_to_number(digits: &[u32]) -> u32 {
    digits.iter().fold(0, |acc, &digit| (acc * 10) + digit)
}

#[derive(Debug)]
pub struct Part {
    digits: Vec<u32>,
    co_ords: Vec<(usize, usize)>,
}

impl Part {
    pub fn value(self: &Self) -> u32 {
        vector_to_number(&self.digits)
    }
}

fn get_no_from_line(line: String, line_no: usize) -> Vec<Part> {
    let mut parts: Vec<Part> = vec![];
    let mut making_number = false;

    for (char_ind, character) in line.chars().enumerate() {
        if character.is_digit(10) && !making_number {
            making_number = true;
            parts.push(Part {
                digits: vec![],
                co_ords: vec![],
            })
        }
        if character.is_digit(10) {
            let current_part = parts.last_mut().unwrap();
            current_part.co_ords.push((char_ind, line_no));
            current_part.digits.push(character.to_digit(10).unwrap());
        } else {
            making_number = false;
        }
    }
    parts
}

fn get_adjacent_points(center: (usize, usize), width: i32, height: i32) -> Vec<(usize, usize)> {
    let mut adjacent_points = Vec::new();

    // Define the directions: up, down, left, right
    let directions: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (1, -1),
        (1, 1),
        (-1, 1),
    ];

    for &(dx, dy) in &directions {
        let new_x = center.0 as i32 + dx;
        let new_y = center.1 as i32 + dy;

        // Check if the new point is within bounds
        if new_x >= 0 && new_x < width && new_y >= 0 && new_y < height {
            adjacent_points.push((new_x as usize, new_y as usize));
        }
    }

    adjacent_points
}

pub fn part1() -> u32 {
    let lines = lines_from_file("./src/inputs/day3.txt");
    let height = lines.len() as i32;
    let width = lines.first().unwrap().len() as i32;
    let all_numbers: Vec<Part> = lines
        .clone()
        .into_iter()
        .enumerate()
        .flat_map(|(line_no, line)| get_no_from_line(line, line_no))
        .collect();

    let mut co_ord_to_part_map: HashMap<&(usize, usize), usize> = HashMap::new();

    for (part_index, part) in all_numbers.iter().enumerate() {
        for co_ord in &part.co_ords {
            co_ord_to_part_map.insert(co_ord, part_index);
        }
    }
    debug!("mapp {:?}", co_ord_to_part_map);

    let mut indices_to_sum: HashSet<&usize> = HashSet::new();
    let mut weird_chars: HashSet<char> = HashSet::new();
    for (line_index, line) in lines.into_iter().enumerate() {
        for (character_index, character) in line.chars().enumerate() {
            if !character.is_alphanumeric() && !(character == '.') {
                weird_chars.insert(character);
                // Check for adjacent parts
                let points = get_adjacent_points((character_index, line_index), width, height);
                for p in points {
                    indices_to_sum.insert(co_ord_to_part_map.get(&p).unwrap_or(&0));
                }
            }
        }
    }
    debug!("chars {:?}", weird_chars);
    debug!("wh {:?} {}", width,height);
    indices_to_sum.into_iter().fold(0, |acc, x| acc + all_numbers[*x].value())
}
pub fn part2() {}

// Iterate till you hit a symbol.
// Check the adjacent co-ords
// If any are a number, trace to r/l till you get the full number.
// replace the number with dots

// Iterate and extract all numbers.
// For each make an ID, and store is_adj=false and adj_co_ords
// If symbol in adj_coords, update
