use super::Solve;
use std::collections::BinaryHeap;
/// # 2558. Take Gifts From the Richest Pile
///
/// You are given an integer array `gifts` denoting the number of gifts in various piles.
/// Every second, you do the following:
///
/// - Choose the pile with the maximum number of gifts.
/// - If there is more than one pile with the maximum number of gifts, choose any.
/// - Reduce the number of gifts in the pile to the floor of the square root of the original number of gifts in the pile.
///
/// Return the number of gifts remaining after `k` seconds.
pub struct PickGifts;

impl PickGifts {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut numbers: BinaryHeap<i64> = gifts.iter().map(|&n| n as i64).collect();
        for _ in 0..k {
            if let Some(mut max_number) = numbers.peek_mut() {
                match *max_number {
                    1 => break,
                    _ => *max_number = max_number.isqrt(),
                }
            }
        }
        numbers.iter().sum()
    }
}

impl Solve<(Vec<i32>, i32), i64> for PickGifts {
    fn solve(input: (Vec<i32>, i32)) -> i64 {
        Self::pick_gifts(input.0, input.1)
    }
}