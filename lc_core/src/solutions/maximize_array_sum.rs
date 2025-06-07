// lc_core/src/solutions/maximize_array_sum.rs
use super::Solve;

pub struct MaximizeArraySum;

impl MaximizeArraySum {
    // LeetCode style solution
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut remaining_ops = k;
        for num in nums.iter_mut() {
            if *num < 0 && remaining_ops > 0 {
                *num = -*num;
                remaining_ops -= 1;
            }
        }

        if remaining_ops > 0 && remaining_ops % 2 != 0 {
            let min_idx = nums.iter().enumerate()
                .min_by_key(|&(_, val)| val)
                .map(|(idx, _)| idx)
                .unwrap_or(0);
            nums[min_idx] = -nums[min_idx];
        }

        nums.iter().sum()
    }
}

impl Solve<(Vec<i32>, i32), i32> for MaximizeArraySum {
    fn solve((nums, k): (Vec<i32>, i32)) -> i32 {
        // Use the LeetCode-style solution
        Self::maximize_sum(nums, k)
    }
}
