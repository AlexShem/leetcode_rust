// lc_core/src/solutions/two_sum.rs
use super::Solve;
use std::collections::HashMap;

pub struct TwoSum;

impl TwoSum {
    // LeetCode style solution
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_map.get(&complement) {
                return vec![index, i as i32];
            }
            num_map.insert(num, i as i32);
        }

        vec![] // Return empty vector if no solution found
    }
}

impl Solve<(Vec<i32>, i32), Option<(usize, usize)>> for TwoSum {
    fn solve((nums, target): (Vec<i32>, i32)) -> Option<(usize, usize)> {
        // Use the LeetCode-style solution and convert the result
        let result = Self::two_sum(nums, target);
        if result.len() == 2 {
            return Some((result[0] as usize, result[1] as usize));
        }
        None
    }
}
