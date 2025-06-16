use std::collections::HashMap;
use super::Solve;

/// # 3318. Find X-Sum of All K-Long Subarrays I
///
/// You are given an array `nums` of `n` integers and two integers `k` and `x`.
///
/// The **x-sum** of an array is calculated by the following procedure:
///
/// - Count the occurrences of all elements in the array.
/// - Keep only the occurrences of the top `x` most frequent elements.
/// If two elements have the same number of occurrences,
/// the element with the bigger value is considered more frequent.
/// - Calculate the sum of the resulting array.
///
/// **Note** that if an array has less than `x` distinct elements,
/// its **x-sum** is the sum of the array.
///
/// Return an integer array `answer` of length `n - k + 1`
/// where `answer[i]` is the **x-sum** of the subarray `nums[i...i + k - 1]`.
pub struct FindXSum;

impl FindXSum {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let answer_len = nums.len() - k as usize + 1;
        let x = x as usize;
        let mut answer: Vec<i32> = Vec::with_capacity(answer_len);

        for i in 0..answer_len {
            let subarray = &nums[i..=i + k as usize - 1];

            let mut freq_map: HashMap<i32, usize> = HashMap::new();
            for num in subarray {
                *freq_map.entry(*num).or_insert(0) += 1;
            }

            let mut freq_pairs: Vec<(i32, usize)> = freq_map.into_iter().collect();
            freq_pairs.sort_by(|a, b| {
                b.1.cmp(&a.1).then(b.0.cmp(&a.0))
            });

            let x_sum: i32 = if freq_pairs.len() <= x {
                subarray.iter().sum()
            } else {
                freq_pairs.iter().take(x)
                    .map(|(val, count)| {
                        val * (*count as i32)
                    })
                    .sum()
            };

            answer.push(x_sum);
        }
        answer
    }
}

impl Solve<(Vec<i32>, i32, i32), Vec<i32>> for FindXSum {
    fn solve(input: (Vec<i32>, i32, i32)) -> Vec<i32> {
        Self::find_x_sum(input.0, input.1, input.2)
    }
}