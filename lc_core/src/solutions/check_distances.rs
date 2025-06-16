use std::collections::{HashMap, HashSet};
use super::Solve;

/// # 2399. Check Distances Between Same Letters
///
/// You are given a **0-indexed** string `s` consisting of only lowercase English letters, 
/// where each letter in `s` appears **exactly twice**.
/// You are also given a 0-indexed integer array `distance` of length `26`.
///
/// Each letter in the alphabet is numbered from `0` to `25`
/// (i.e. `'a' -> 0`, `'b' -> 1`, `'c' -> 2`, ... , `'z' -> 25`).
///
/// In a **well-spaced string**, the number of letters between the two occurrences 
/// of the `i`th letter is `distance[i]`. If the `i`th letter does not appear in `s`, 
/// then `distance[i]` can be ignored.
///
/// *Return `true` if s is a **well-spaced** string, otherwise return `false`.*
pub struct CheckDistances;

impl CheckDistances {
    pub fn check_distances(s: String, distance: Vec<i32>) -> bool {
        let letters: HashSet<char> = s.chars().collect();
        let distance_hash: HashMap<char, i32> = letters
            .iter()
            .map(|ch| {
                let left_idx = s.find(*ch).unwrap();
                let right_idx = s.rfind(*ch).unwrap();
                (*ch, (right_idx - left_idx - 1) as i32)
            })
            .collect::<HashMap<char, i32>>();
        distance_hash
            .iter()
            .fold(true, |res, (ch, dist)| {
                res && distance[(*ch as u8 - b'a') as usize] == *dist
            })
    }
}

impl Solve<(String, Vec<i32>), bool> for CheckDistances {
    fn solve(input: (String, Vec<i32>)) -> bool {
        Self::check_distances(input.0, input.1)
    }
}