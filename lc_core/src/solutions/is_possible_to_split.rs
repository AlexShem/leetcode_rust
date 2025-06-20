use std::collections::HashMap;
use super::Solve;

/// # 3046. Split the Array
/// You are given an integer array `nums` of even length.
/// You have to split the array into two parts `nums1` and `nums2` such that:
///
/// - `nums1.length == nums2.length == nums.length / 2`.
/// - `nums1` should contain distinct elements.
/// - `nums2` should also contain distinct elements.
///
/// Return `true` if it is possible to split the array, and `false` otherwise.
pub struct IsPossibleToSplit;

impl IsPossibleToSplit {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        let mut seen: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            if !seen.contains_key(&num) {
                seen.insert(num, 1);
            } else {
                if let Some(val) = seen.get(&num) {
                    match val {
                        1 => seen.entry(num).and_modify(|v| *v += 1).or_insert(2),
                        _ => return false
                    };
                }
            }
        }
        true
    }
}

impl Solve<Vec<i32>, bool> for IsPossibleToSplit {
    fn solve(input: Vec<i32>) -> bool {
        Self::is_possible_to_split(input)
    }
}