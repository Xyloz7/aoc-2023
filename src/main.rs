pub mod days;
use days::common;
use days::dayone;
use days::daytwo;
use log::{debug, error, info, log_enabled, Level};
use std::env;

fn get_day() -> u32 {
    // Read a usize from user input

    let args: Vec<String> = env::args().collect();

    // Check if at least one argument is provided
    if args.len() < 2 {
        println!("Please provide a usize value as a command-line argument.");
        return 0;
    }

    // Parse the input argument into a usize
    let usize_value: usize = match args[1].trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return 0;
        }
    };

    // Convert usize to u32 using as
    let u32_value: u32 = usize_value as u32;
    u32_value
}
fn main() {
    env_logger::init();

    let day = get_day();
    match day {
        1 => {
            println!("Day one part 1 answer is {}", dayone::day_one());
            println!("Day one part 2 answer is {}", dayone::day_one_part2());
        }
        2 => {
            println!("Day one part 1 answer is {}", daytwo::day2_part1());
        }
        _ => println!("Invalid day chosen!"),
    }
}
