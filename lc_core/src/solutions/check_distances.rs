use std::collections::{HashMap, HashSet};
use super::Solve;

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