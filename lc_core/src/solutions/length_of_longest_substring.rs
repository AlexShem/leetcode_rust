use super::Solve;

pub struct LengthOfLongestSubstring;

impl LengthOfLongestSubstring {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut window = String::new();
        let mut longest_len = 0;

        for pos in 0..chars.len() {
            let char = chars[pos];
            if let Some(index) = window.find(char) {
                longest_len = longest_len.max(window.len());
                if index + 1 == window.len() {
                    window = String::new();
                } else {
                    window = window[index + 1..].to_string();
                }
            }
            window.push(char);
        }

        longest_len.max(window.len()) as i32
    }
}

impl Solve<String, i32> for LengthOfLongestSubstring {
    fn solve(input: String) -> i32 {
        Self::length_of_longest_substring(input)
    }
}