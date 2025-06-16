use super::Solve;
use std::collections::HashMap;

pub struct CountBalls;

impl CountBalls {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let mut boxes: HashMap<i32, i32> = HashMap::new();
        for ball in low_limit..=high_limit {
            let mut digit_sum = 0;
            let mut num = ball.clone();
            while num > 0 {
                digit_sum += num % 10;
                num /= 10;
            }
            *boxes.entry(digit_sum).or_insert(0) += 1;
        }

        *boxes
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|entry| entry.1)
            .unwrap()
    }
}

impl Solve<(i32, i32), i32> for CountBalls {
    fn solve(input: (i32, i32)) -> i32 {
        Self::count_balls(input.0, input.1)
    }
}