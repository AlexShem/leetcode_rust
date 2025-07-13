use super::Solve;

/// # 1390. Four Divisors
///
/// Medium
///
/// Given an integer array nums, return the sum of divisors of the integers in that array that have exactly four divisors. If there is no such integer in the array, return 0.

pub struct SumFourDivisors;

impl SumFourDivisors {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        nums.iter()
            .filter_map(|num| {
                let dividers = Self::divisors(*num);
                if dividers.len() == 4 {
                    return Some(dividers.iter().sum::<i32>());
                }
                return None;
            })
            .sum()
    }

    fn divisors(num: i32) -> Vec<i32> {
        let limit = num.isqrt();
        let mut divisors = Vec::new();

        for i in 1..=limit {
            if num % i == 0 {
                divisors.push(i);
                if i * i != num {
                    let complement = num / i;
                    divisors.push(complement);
                }
            }
        }

        divisors
    }
}

impl Solve<Vec<i32>, i32> for SumFourDivisors {
    fn solve(input: Vec<i32>) -> i32 {
        Self::sum_four_divisors(input)
    }
}
