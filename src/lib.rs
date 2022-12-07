mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("1:");
    let contents = fs::read_to_string("./input/day1.txt")?;
    println!("  a: {}", day1::solve_a(&contents));
    println!("  b: {}", day1::solve_b(&contents));
    println!();

    let contents = fs::read_to_string("./input/day2.txt")?;
    println!("2:");
    println!("  a: {}", day2::solve_a(&contents));
    println!("  b: {}", day2::solve_b(&contents));
    println!();

    let contents = fs::read_to_string("./input/day3.txt")?;
    println!("3:");
    println!("  a: {}", day3::solve_a(&contents));
    println!("  b: {}", day3::solve_b(&contents));
    println!();

    let contents = fs::read_to_string("./input/day4.txt")?;
    println!("4:");
    let a = day4::solve_a(&contents);
    if let Ok(a) = a {
        println!("  a: {}", a);
    } else {
        println!("  a: {:?}", a);
    }
    println!("  b: {}", day4::solve_b(&contents).unwrap_or(-1));
    println!();

    let contents = fs::read_to_string("./input/day5.txt")?;
    println!("5:");
    let a = day5::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {}", a),
        Err(a) => println!("  a: {}", a),
    };
    let b = day5::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {}", b),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day6.txt")?;
    println!("5:");
    let a = day6::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {}", a),
        Err(a) => println!("  a: {}", a),
    };
    let b = day6::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {}", b),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    Ok(())
}
