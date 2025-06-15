use super::Solve;

pub struct CountAsterisks;

impl CountAsterisks {
    pub fn count_asterisks(s: String) -> i32 {
        let mut counter = 0;
        let mut in_group = false;

        for ch in s.chars() {
            if !in_group && ch == '*' {
                counter += 1;
            } else if ch == '|' {
                in_group = !in_group;
            }
        }
        counter
    }
}

impl Solve<String, i32> for CountAsterisks {
    fn solve(input: String) -> i32 {
        Self::count_asterisks(input)
    }
}