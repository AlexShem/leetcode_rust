use lc_core::solutions::{Solve, is_palindrome::IsPalindrome as S};

#[test]
fn test_1() {
    let num = 121;
    let expected = true;
    assert_eq!(S::solve(num), expected);
}

#[test]
fn test_2() {
    let num = -121;
    let expected = false;
    assert_eq!(S::solve(num), expected);
}

#[test]
fn test_3() {
    let num = 10;
    let expected = false;
    assert_eq!(S::solve(num), expected);
}