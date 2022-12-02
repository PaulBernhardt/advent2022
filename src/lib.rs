use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input.txt")?;

    println!("a: {}", solve_a(&contents));
    println!("b: {}", solve_b(&contents));
    return Ok(());
}

pub fn solve_a(contents: &str) -> i32 {
    let mut totals = Vec::new();
    let mut current = 0;
    for line in contents.lines() {
        let result = line.parse::<i32>();
        match result {
            Ok(num) => {
                // println!("{}", num);
                current = current + num
            }
            Err(_) => {
                totals.push(current);
                current = 0
            }
        }
    }
    totals.sort_unstable_by(|a, b| b.cmp(a));

    return totals[0];
}

pub fn solve_b(contents: &str) -> i32 {
    let mut totals = Vec::new();
    let mut current = 0;
    for line in contents.lines() {
        let result = line.parse::<i32>();
        match result {
            Ok(num) => {
                // println!("{}", num);
                current = current + num
            }
            Err(_) => {
                totals.push(current);
                current = 0
            }
        }
    }
    totals.sort_unstable_by(|a, b| b.cmp(a));

    return totals[0] + totals[1] + totals[2];
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    return results;
}
