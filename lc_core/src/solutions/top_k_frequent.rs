use std::collections::{BinaryHeap, HashMap};
use super::Solve;

/// # 347. Top K Frequent Elements
/// **Difficulty**: *Medium*
///
/// Given an integer array `nums` and an integer `k`, return the `k` *most frequent elements*.
/// You may return the answer in any order.
pub struct TopKFrequent;

impl TopKFrequent {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            freq_map
                .entry(num)
                .and_modify(|freq| *freq += 1)
                .or_insert(1);
        }
        let mut top_freq: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        for (num, freq) in freq_map.drain() {
            top_freq.push((freq, num))
        }
        let mut top_k: Vec<i32> = Vec::with_capacity(k as usize);
        for _ in 0..k {
            top_k.push(top_freq.pop().map(|(_, num)| num).unwrap());
        }
        top_k
    }
}

impl Solve<(Vec<i32>, i32), Vec<i32>> for TopKFrequent {
    fn solve(input: (Vec<i32>, i32)) -> Vec<i32> {
        Self::top_k_frequent(input.0, input.1)
    }
}