use crate::solutions::Solve;

pub struct ReverseString;

impl ReverseString {
    pub fn reverse_string(s: &mut Vec<char>) {
        s.reverse()
    }
}

impl Solve<&mut Vec<char>, ()> for ReverseString {
    fn solve(input: &mut Vec<char>) -> () {
        Self::reverse_string(input);
    }
}