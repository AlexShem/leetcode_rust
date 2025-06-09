use super::Solve;

pub struct MinimumAverage;

impl MinimumAverage {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        let mut min_avg = f64::MAX;
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            min_avg = min_avg.min(((nums[left] + nums[right]) as f64) / 2.0);
            left += 1;
            right -= 1;
        }
        min_avg
    }
}

impl Solve<Vec<i32>, f64> for MinimumAverage {
    fn solve(input: Vec<i32>) -> f64 {
        Self::minimum_average(input)
    }
}