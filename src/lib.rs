mod day1;
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

    Ok(())
}
