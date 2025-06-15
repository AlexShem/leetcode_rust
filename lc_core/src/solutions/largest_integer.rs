use super::Solve;
use std::collections::BinaryHeap;

pub struct LargestInteger;

impl LargestInteger {
    fn largest_integer(mut num: i32) -> i32 {
        let mut even_odd = [BinaryHeap::new(), BinaryHeap::new()];
        let mut digits: Vec<i32> = vec![];

        while num > 0 {
            let digit = (num % 10) as usize;
            digits.push(digit as i32);
            even_odd[digit % 2].push(digit as i32);
            num /= 10;
        }

        let res = digits
            .iter()
            .rev()
            .fold(0, |acc, &x| acc * 10 + even_odd[(x % 2) as usize].pop().unwrap());
        res
    }
}

impl Solve<i32, i32> for LargestInteger {
    fn solve(input: i32) -> i32 {
        LargestInteger::largest_integer(input)
    }
}