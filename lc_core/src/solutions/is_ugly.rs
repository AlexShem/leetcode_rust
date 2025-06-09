use super::Solve;

pub struct IsUgly;

impl IsUgly {
    pub fn is_ugly(n: i32) -> bool {
        if n <= 0 { return false; }
        if n == 1 { return true; }
        let mut res = n;

        let primes = vec![2, 3, 5];
        for p in primes.iter() {
            while res % p == 0 {
                res /= p;
            }
        }
        match res {
            1 => true,
            _ => false,
        }
    }
}

impl Solve<i32, bool> for IsUgly {
    fn solve(input: i32) -> bool {
        Self::is_ugly(input)
    }
}