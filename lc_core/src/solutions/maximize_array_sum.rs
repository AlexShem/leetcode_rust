// lc_core/src/solutions/maximize_array_sum.rs
use super::Solve;

pub struct MaximizeArraySum;

impl MaximizeArraySum {
    // LeetCode style solution
    pub fn maximize_sum(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();

        for i in 0..n {
            if nums[i] < 0 && k > 0 {
                nums[i] = -nums[i];
                k -= 1;
            }
        }

        nums.sort();
        if k > 0 && k % 2 != 0 {
            nums[0] = -nums[0];
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
