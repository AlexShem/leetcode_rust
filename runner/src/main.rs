// runner/src/main.rs
//! `cargo run -p runner -- two_sum "2 7 11 15" 9`
//!
//! `cargo run -p runner -- maximize_array_sum -- "-1 -2 3 13 -4 -6" 2`

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
        "maximize_array_sum" => {
            let nums: Vec<i32> = args.input[0]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let k: i32 = args.input[1].parse().unwrap();
            let ans = lc_core::solutions::maximize_array_sum::MaximizeArraySum::solve((nums, k));
            println!("{ans}");
        }
        _ => eprintln!("unknown problem"),
    }
}
