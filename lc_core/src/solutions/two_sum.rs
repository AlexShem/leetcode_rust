// lc_core/src/solutions/two_sum.rs
use super::Solve;
use std::collections::HashMap;

pub struct TwoSum;

impl Solve<(Vec<i32>, i32), Option<(usize, usize)>> for TwoSum {
    fn solve((nums, target): (Vec<i32>, i32)) -> Option<(usize, usize)> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_map.get(&complement) {
                return Some((index as usize, i));
            }
            num_map.insert(num, i as i32);
        }
        None
    }
}
