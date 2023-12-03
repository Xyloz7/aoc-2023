use log::debug;
use std::collections::HashMap;

use crate::common::lines_from_file;

fn is_set_possible(cubeset: &str, totals: &HashMap<&str, u32>) -> bool {
    let cubes = cubeset.split(",");

    for cube in cubes {
        let num_cubes = cube
            .strip_prefix(" ")
            .unwrap()
            .split(" ")
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let cube_colour = cube.split(" ").last().unwrap();

        let is_possible_set = num_cubes < totals[cube_colour];
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
        debug!(
            "Set {} is possible? {}",
            cubeset,
            is_set_possible(cubeset, totals)
        );
        if !is_set_possible(cubeset, totals) {
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
        debug!(
            "Game {} is possible? {}\n",
            line,
            is_game_possible(line.clone(), &totals)
        );

        if is_game_possible(line, &totals) {
            id_sum += i + 1
        }
    }

    debug!("Sum of valid Ids is {}", id_sum);
    id_sum
}
