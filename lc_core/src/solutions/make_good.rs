use super::Solve;

pub struct MakeGood;

impl MakeGood {
    pub fn make_good(s: String) -> String {
        if s.is_empty() { return "".to_string(); }

        let mut result: String = String::with_capacity(s.len());
        for char in s.chars() {
            if let Some(last_char) = result.chars().last() {
                if (last_char as i32 - char as i32).abs() == 32 {
                    result.pop();
                    continue;
                }
            }
            result.push(char);
        }
        result
    }
}

impl Solve<String, String> for MakeGood {
    fn solve(input: String) -> String {
        Self::make_good(input)
    }
}