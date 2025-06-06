// lc_core/tests/maximize_array_sum_test.rs
use lc_core::solutions::{Solve, maximize_array_sum::MaximizeArraySum as S};

#[test]
fn test_1() {
    let nums = vec![4, 2, 3];
    let k = 1;
    let expected = 5;
    assert_eq!(S::solve((nums, k)), expected);
}

#[test]
fn test_2() {
    let nums = vec![3, -1, 0, 2];
    let k = 3;
    let expected = 6;
    assert_eq!(S::solve((nums, k)), expected);
}

#[test]
fn test_3() {
    let nums = vec![2, -3, -1, 5, -4];
    let k = 2;
    let expected = 13;
    assert_eq!(S::solve((nums, k)), expected);
}
