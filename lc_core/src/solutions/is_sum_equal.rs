use super::Solve;

pub struct IsSumEqual;

impl IsSumEqual {
    fn word_to_number(word: &String) -> i32 {
        let number: String = word
            .chars()
            .map(|ch| {
                if ch >= 'a' && ch <= 'j' {
                    ((ch as u8 - b'a') + b'0') as char
                } else {
                    ch
                }
            })
            .collect();
        number.parse().expect("Cannot parse the string to number.")
    }
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let first_num = Self::word_to_number(&first_word);
        let second_num = Self::word_to_number(&second_word);
        let target_num = Self::word_to_number(&target_word);

        if first_num + second_num == target_num {
            true
        } else {
            false
        }
    }
}

impl Solve<(String, String, String), bool> for IsSumEqual {
    fn solve(input: (String, String, String)) -> bool {
        Self::is_sum_equal(input.0, input.1, input.2)
    }
}