use super::Solve;

/// # 3133. Minimum Array End
/// **Difficulty**: *Medium*
///
/// You are given two integers `n` and `x`.
/// You have to construct an array of positive integers `nums` of size `n`
/// where for every `0 <= i < n - 1`, `nums[i + 1]` is greater than `nums[i]`,
/// and the result of the bitwise AND operation between all elements of nums is `x`.
///
/// Return the minimum possible value of `nums[n - 1]`.
pub struct MinEnd;

impl MinEnd {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let x = x as i64;
        let mut res: i64 = x;
        for _ in 0..n - 1 {
            res = (res + 1) | x;
        }
        res
    }
}

impl Solve<(i32, i32), i64> for MinEnd {
    fn solve(input: (i32, i32)) -> i64 {
        Self::min_end(input.0, input.1)
    }
}