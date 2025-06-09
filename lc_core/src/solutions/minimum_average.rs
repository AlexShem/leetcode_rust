use super::Solve;

pub struct MinimumAverage;

impl MinimumAverage {
    /// Calculates the minimum average of paired elements from opposite ends of the sorted array
    ///
    /// # Arguments
    /// * `nums` - Vector of integers to process
    ///
    /// # Returns
    /// The minimum average of any paired elements
    pub fn minimum_average(nums: Vec<i32>) -> f64 {
        if nums.len() < 2 {
            return 0.0;
        }

        let mut sorted = nums;
        sorted.sort_unstable();

        (0..sorted.len() / 2)
            .map(|i| {
                let left = sorted[i];
                let right = sorted[sorted.len() - 1 - i];
                (left as f64 + right as f64) / 2.0
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(f64::MAX)
    }
}

impl Solve<Vec<i32>, f64> for MinimumAverage {
    fn solve(input: Vec<i32>) -> f64 {
        Self::minimum_average(input)
    }
}