use lc_core::solutions::Solve;
use lc_core::solutions::length_of_longest_substring::LengthOfLongestSubstring as S;

#[test]
fn length_of_longest_substring_lc1() {
    let input = String::from("abcabcbb");
    let result = S::solve(input);
    let expected = 3;

    assert_eq!(result, expected);
}

#[test]
fn length_of_longest_substring_lc2() {
    let input = String::from("bbbbb");
    let result = S::solve(input);
    let expected = 1;

    assert_eq!(result, expected);
}

#[test]
fn length_of_longest_substring_lc3() {
    let input = String::from("pwwkew");
    let result = S::solve(input);
    let expected = 3;

    assert_eq!(result, expected);
}

#[test]
fn length_of_longest_substring_lc4() {
    let input = String::from(" ");
    let result = S::solve(input);
    let expected = 1;

    assert_eq!(result, expected);
}
