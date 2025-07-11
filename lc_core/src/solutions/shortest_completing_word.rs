use std::collections::HashMap;
use crate::solutions::Solve;

/// # 748. Shortest Completing Word
///
/// Easy
///
/// Given a string licensePlate and an array of strings words, find the shortest completing word in words.
///
/// A completing word is a word that contains all the letters in licensePlate. Ignore numbers and spaces in licensePlate, and treat letters as case insensitive. If a letter appears more than once in licensePlate, then it must appear in the word the same number of times or more.
///
/// For example, if licensePlate = "aBc 12c", then it contains letters 'a', 'b' (ignoring case), and 'c' twice. Possible completing words are "abccdef", "caaacab", and "cbca".
///
/// Return the shortest completing word in words. It is guaranteed an answer exists. If there are multiple shortest completing words, return the first one that occurs in words.
pub struct ShortestCompletingWord;

impl ShortestCompletingWord {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let letters_count = Self::letter_count(&license_plate);

        let mut shortest_idx: usize = 0;
        let mut shortest_len = usize::MAX;

        'outer: for (idx, word) in words.iter().enumerate() {
            let word_letter_count = Self::letter_count(word);
            for key in letters_count.keys() {
                if let Some(value) = word_letter_count.get(key) {
                    if letters_count.get(key).unwrap() > value {
                        continue 'outer;
                    }
                } else {
                    continue 'outer;
                }
            }
            let word_len = word.len();
            if word_len < shortest_len {
                shortest_idx = idx;
                shortest_len = word_len;
            }
        }

        words[shortest_idx].clone()
    }

    fn letter_count(word: &String) -> HashMap<char, u32> {
        let word = word.to_lowercase();
        let mut letters_count: HashMap<char, u32> = HashMap::with_capacity(word.len());
        for ch in word.chars() {
            if ch.is_alphabetic() {
                letters_count.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
            } else {
                continue;
            }
        }
        letters_count
    }
}

impl Solve<(String, Vec<String>), String> for ShortestCompletingWord {
    fn solve(input: (String, Vec<String>)) -> String {
        Self::shortest_completing_word(input.0, input.1)
    }
}