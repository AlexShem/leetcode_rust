use lc_core::solutions::{Solve, is_sum_equal::IsSumEqual as S};

#[test]
fn test_1() {
    let first_word = String::from("acb");
    let second_word = String::from("cba");
    let target_word = String::from("cdb");
    let expected = true;
    assert_eq!(S::solve((first_word, second_word, target_word)), expected)
}

#[test]
fn test_2() {
    let first_word = String::from("aaa");
    let second_word = String::from("a");
    let target_word = String::from("aab");
    let expected = false;
    assert_eq!(S::solve((first_word, second_word, target_word)), expected)
}

#[test]
fn test_3() {
    let first_word = String::from("aaa");
    let second_word = String::from("a");
    let target_word = String::from("aaaa");
    let expected = true;
    assert_eq!(S::solve((first_word, second_word, target_word)), expected)
}
