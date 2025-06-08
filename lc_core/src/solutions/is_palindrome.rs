use super::Solve;

pub struct IsPalindrome;

impl IsPalindrome {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 { return false; };

        let s = x.to_string();
        s.chars().eq(s.chars().rev())
    }
}

impl Solve<i32, bool> for IsPalindrome {
    fn solve(input: i32) -> bool {
        Self::is_palindrome(input)
    }
}