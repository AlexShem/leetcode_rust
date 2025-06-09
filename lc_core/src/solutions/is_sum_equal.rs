use super::Solve;

pub struct IsSumEqual;

impl IsSumEqual {
    fn word_to_number(word: &String) -> Option<i32> {
        if !word.chars().all(|ch| ('a'..='j').contains(&ch)) {
            return None;
        }
        let number: String = word
            .chars()
            .map(|ch| ((ch as u8 - b'a') + b'0') as char)
            .collect();
        number.parse::<i32>().ok()
    }
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        Self::word_to_number(&first_word)
            .and_then(|first_num| {
                Self::word_to_number(&second_word)
                    .map(|second_num| (first_num, second_num))
            })
            .and_then(|(first_num, second_num)| {
                Self::word_to_number(&target_word)
                    .map(|target_num| first_num + second_num == target_num)
            })
            .unwrap_or(false)
    }
}

impl Solve<(String, String, String), bool> for IsSumEqual {
    fn solve(input: (String, String, String)) -> bool {
        Self::is_sum_equal(input.0, input.1, input.2)
    }
}