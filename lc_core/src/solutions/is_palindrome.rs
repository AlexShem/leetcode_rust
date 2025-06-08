use super::Solve;

pub struct IsPalindrome;

impl IsPalindrome {
    pub fn is_palindrome(x: i32) -> bool {
        let number = x.to_string();
        let n = number.len();
        let half = n / 2;
        let inverse: Vec<char> = number.chars().collect();

        for (idx, digit) in number.chars().enumerate() {
            if idx >= half {
                return true;
            }
            if digit != *inverse.iter().nth_back(idx).unwrap() {
                return false;
            }
        }

        true
    }
}

impl Solve<i32, bool> for IsPalindrome {
    fn solve(input: i32) -> bool {
        Self::is_palindrome(input)
    }
}