use crate::solutions::Solve;

pub struct LargestGoodInteger;

impl LargestGoodInteger {
    pub fn largest_good_integer(num: String) -> String {
        let cases = vec!["999", "888", "777", "666", "555", "444", "333", "222", "111", "000"];
        for case in cases {
            if let Some(_) = num.find(case) {
                return case.to_string();
            }
        }
        "".to_string()
    }
}

impl Solve<String, String> for LargestGoodInteger {
    fn solve(input: String) -> String {
        Self::largest_good_integer(input)
    }
}