use super::Solve;

pub struct XorOperation;

impl XorOperation {
    pub fn xor_operation(n: i32, start: i32) -> i32 {
        (0..n)
            .map(|i| start + 2 * i)
            .reduce(|acc, e| acc ^ e)
            .unwrap_or(0)
    }
}

impl Solve<(i32, i32), i32> for XorOperation {
    fn solve(input: (i32, i32)) -> i32 {
        Self::xor_operation(input.0, input.1)
    }
}