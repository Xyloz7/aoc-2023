use log::debug;
use std::collections::HashMap;

use crate::common::lines_from_file;

fn is_set_possible(cubeset: &str, totals: &HashMap<&str, u32>) -> bool {
    let cubes = cubeset.split(",");

    for cube in cubes {
        let num_cubes = cube
            .strip_prefix(" ")
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or_default();

        let cube_colour = cube.split(" ").last().unwrap();

        let is_possible_set = num_cubes <= totals[cube_colour];
        if !is_possible_set {
            return false;
        }
    }
    true
}

fn is_game_possible(line: String, totals: &HashMap<&str, u32>) -> bool {
    let cubesets = line
        .split(":")
        .last()
        .expect("All lines should start with Game :")
        .split(";");

    for cubeset in cubesets {
        let set_possible = is_set_possible(cubeset, totals);
        debug!("Set {} is possible? {}", cubeset, set_possible);
        if !set_possible {
            return false;
        }
    }
    true
}

pub fn day2_part1() -> usize {
    let mut totals: HashMap<&str, u32> = HashMap::new();
    totals.insert("red", 12);
    totals.insert("green", 13);
    totals.insert("blue", 14);

    let mut id_sum = 0;

    let lines = lines_from_file("./src/inputs/day2.txt");
    for (i, line) in lines.into_iter().enumerate() {
        let game_possible = is_game_possible(line.clone(), &totals);
        debug!("Game {} is possible? {}\n", i + 1, game_possible);

        if game_possible {
            id_sum += i + 1
        }
    }

    debug!("Sum of valid Ids is {}", id_sum);
    id_sum
}

fn power_of_game(line: String) -> u32 {
    let mut colour_counts: HashMap<&str, u32> = HashMap::new();
    colour_counts.insert("red", 0);
    colour_counts.insert("green", 0);
    colour_counts.insert("blue", 0);

    let cubesets = line
        .split(":")
        .last()
        .expect("All lines should start with Game :")
        .split(";");

    for cubeset in cubesets {
        let cubes = cubeset.split(",");

        for cube in cubes {
            let num_cubes = cube
                .strip_prefix(" ")
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or_default();

            let cube_colour = cube.split(" ").last().unwrap();
            if num_cubes > colour_counts[cube_colour] {
                colour_counts.insert(cube_colour, num_cubes);
            }
        }
    }

    colour_counts.into_iter().fold(1, |acc, (_, v)| acc * v)
}

pub fn day2_part2() -> u32 {
    lines_from_file("./src/inputs/day2.txt")
        .into_iter()
        .fold(0, |acc, l| acc + power_of_game(l))
}
