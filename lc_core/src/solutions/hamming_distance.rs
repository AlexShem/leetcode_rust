use super::Solve;

pub struct HammingDistance;

impl HammingDistance {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

impl Solve<(i32, i32), i32> for HammingDistance {
    fn solve(input: (i32, i32)) -> i32 {
        Self::hamming_distance(input.0, input.1)
    }
}