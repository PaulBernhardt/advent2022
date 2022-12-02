use advent2022::run;
use std::process;

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {e}");
        process::exit(2);
    }
}
