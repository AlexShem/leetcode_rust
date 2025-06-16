use lc_core::solutions::{Solve, find_x_sum::FindXSum as S};

#[test]
fn test_1() {
    let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
    let k = 6;
    let x = 2;
    let expected = vec![6, 10, 12];
    let result = S::solve((nums, k, x));
    assert_eq!(result, expected)
}

#[test]
fn test_2() {
    let nums = vec![3, 8, 7, 8, 7, 5];
    let k = 2;
    let x = 2;
    let expected = vec![11, 15, 15, 15, 12];
    let result = S::solve((nums, k, x));
    assert_eq!(result, expected)
}