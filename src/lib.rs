mod day1;
mod day2;
mod day3;

use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("1:");
    let contents = fs::read_to_string("./input/day1.txt")?;
    println!("  a: {}", day1::solve_a(&contents));
    println!("  b: {}", day1::solve_b(&contents));
    println!("");

    let contents = fs::read_to_string("./input/day2.txt")?;
    println!("2:");
    println!("  a: {}", day2::solve_a(&contents));
    println!("  b: {}", day2::solve_b(&contents));
    println!("");

    let contents = fs::read_to_string("./input/day3.txt")?;
    println!("3:");
    println!("  a: {}", day3::solve_a(&contents));
    println!("  b: {}", day3::solve_b(&contents));
    println!("");

    return Ok(());
}
