use lc_core::solutions::{Solve, remove_digit::RemoveDigit as S};

#[test]
fn test_1() {
    let number = "123".to_string();
    let digit = '3';
    let expected = "12".to_string();
    assert_eq!(S::solve((number, digit)), expected);
}

#[test]
fn test_2() {
    let number = "1231".to_string();
    let digit = '1';
    let expected = "231".to_string();
    assert_eq!(S::solve((number, digit)), expected);
}

#[test]
fn test_3() {
    let number = "551".to_string();
    let digit = '5';
    let expected = "51".to_string();
    assert_eq!(S::solve((number, digit)), expected);
}

#[test]
fn test_4() {
    let number = "133235".to_string();
    let digit = '3';
    let expected = "13325".to_string();
    assert_eq!(S::solve((number, digit)), expected);
}
