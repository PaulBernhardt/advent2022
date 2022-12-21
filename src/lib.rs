mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use std::error::Error;
use std::fs;
use std::time::Instant;

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("1:");
    let contents = fs::read_to_string("./input/day1.txt")?;
    let time = Instant::now();
    println!("  a: {} ({:?})", day1::solve_a(&contents), time.elapsed());
    let time = Instant::now();
    println!("  b: {} ({:?})", day1::solve_b(&contents), time.elapsed());
    println!();

    let contents = fs::read_to_string("./input/day2.txt")?;
    println!("2:");
    let time = Instant::now();
    println!("  a: {} ({:?})", day2::solve_a(&contents), time.elapsed());
    let time = Instant::now();
    println!("  b: {} ({:?})", day2::solve_b(&contents), time.elapsed());

    println!();

    let contents = fs::read_to_string("./input/day3.txt")?;
    println!("3:");
    let time = Instant::now();
    println!("  a: {} ({:?})", day3::solve_a(&contents), time.elapsed());
    let time = Instant::now();
    println!("  b: {} ({:?})", day3::solve_b(&contents), time.elapsed());
    println!();

    let contents = fs::read_to_string("./input/day4.txt")?;
    println!("4:");
    let time = Instant::now();
    let a = day4::solve_a(&contents);
    if let Ok(a) = a {
        println!("  a: {} ({:?})", a, time.elapsed());
    } else {
        println!("  a: {:?}", a);
    }
    let time = Instant::now();
    println!(
        "  b: {} ({:?})",
        day4::solve_b(&contents).unwrap_or(-1),
        time.elapsed()
    );
    println!();

    let contents = fs::read_to_string("./input/day5.txt")?;
    println!("5:");
    let time = Instant::now();
    let a = day5::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day5::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day6.txt")?;
    println!("6:");
    let time = Instant::now();
    let a = day6::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day6::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day7.txt")?;
    println!("7:");
    let time = Instant::now();
    let a = day7::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day7::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day8.txt")?;
    println!("8:");
    let time = Instant::now();
    let a = day8::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day8::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day9.txt")?;
    println!("9:");
    let time = Instant::now();
    let a = day9::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day9::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day10.txt")?;
    println!("10:");
    let time = Instant::now();
    let a = day10::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day10::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day11.txt")?;
    println!("11:");
    let time = Instant::now();
    let a = day11::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day11::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day12.txt")?;
    println!("12:");
    let time = Instant::now();
    let a = day12::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day12::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day13.txt")?;
    println!("13:");
    let time = Instant::now();
    let a = day13::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day13::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day14.txt")?;
    println!("14:");
    let time = Instant::now();
    let a = day14::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day14::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    let contents = fs::read_to_string("./input/day15.txt")?;
    println!("15:");
    let time = Instant::now();
    let a = day15::solve_a(&contents);
    match a {
        Ok(a) => println!("  a: {} ({:?})", a, time.elapsed()),
        Err(a) => println!("  a: {}", a),
    };
    let time = Instant::now();
    let b = day15::solve_b(&contents);
    match b {
        Ok(b) => println!("  b: {} ({:?})", b, time.elapsed()),
        Err(b) => println!("  b: {}", b),
    };
    println!();

    Ok(())
}
