use lc_core::solutions::{Solve, is_ugly::IsUgly as S};

#[test]
fn test_1() {
    let num = 6;
    let expected = true;
    assert_eq!(S::solve(num), expected);
}

#[test]
fn test_2() {
    let num = 1;
    let expected = true;
    assert_eq!(S::solve(num), expected);
}

#[test]
fn test_3() {
    let num = 14;
    let expected = false;
    assert_eq!(S::solve(num), expected);
}

#[test]
fn test_4() {
    let num = 0;
    let expected = false;
    assert_eq!(S::solve(num), expected);
}