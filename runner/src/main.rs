// runner/src/main.rs
//! `cargo run -p runner -- two_sum "2 7 11 15" 9`

use clap::Parser;
use lc_core::solutions::{Solve, two_sum::TwoSum};

#[derive(Parser)]
struct Args {
    /// Problem slug, e.g. two_sum
    problem: String,
    /// Raw input line(s)
    input: Vec<String>,
}

fn main() {
    let args = Args::parse();

    match args.problem.as_str() {
        "two_sum" => {
            let nums: Vec<i32> = args.input[0]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let target: i32 = args.input[1].parse().unwrap();
            let ans = TwoSum::solve((nums, target));
            println!("{ans:?}");
        }
        _ => eprintln!("unknown problem"),
    }
}
