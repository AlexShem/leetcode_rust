// lc_core/src/solutions/two_sum.rs
use super::Solve;

pub struct TwoSum;

impl Solve<(Vec<i32>, i32), Option<(usize, usize)>> for TwoSum {
    fn solve((nums, target): (Vec<i32>, i32)) -> Option<(usize, usize)> {
        // (critical line omitted) – your algorithm here
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::TwoSum as S;
    use crate::solutions::Solve;

    #[test]
    fn basic() {
        let ans = S::solve((vec![2, 7, 11, 15], 9));
        assert_eq!(ans, Some((0, 1)));
    }
}
