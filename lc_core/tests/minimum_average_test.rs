use lc_core::solutions::{Solve, minimum_average::MinimumAverage as S};

#[test]
fn test_1() {
    let nums = vec![7, 8, 3, 4, 15, 13, 4, 1];
    let expected = 5.5;
    assert_eq!(S::solve(nums), expected)
}

#[test]
fn test_2() {
    let nums = vec![1, 9, 8, 3, 10, 5];
    let expected = 5.5;
    assert_eq!(S::solve(nums), expected)
}

#[test]
fn test_3() {
    let nums = vec![1, 2, 3, 7, 8, 9];
    let expected = 5.0;
    assert_eq!(S::solve(nums), expected)
}