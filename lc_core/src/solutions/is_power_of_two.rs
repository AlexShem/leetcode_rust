use super::Solve;

pub struct IsPowerOfTwo;

impl IsPowerOfTwo {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }

        let mut n = n;
        while n & 1 == 0 {
            n >>= 1;
        }
        if n > 1 {
            false
        } else {
            true
        }
    }
}

impl Solve<i32, bool> for IsPowerOfTwo {
    fn solve(input: i32) -> bool {
        Self::is_power_of_two(input)
    }
}