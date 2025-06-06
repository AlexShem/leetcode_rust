// lc_core/tests/two_sum_test.rs
use lc_core::solutions::{Solve, two_sum::TwoSum as S};


#[test]
fn test_1() {
    let ans = S::solve((vec![2, 7, 11, 15], 9));
    assert_eq!(ans, Some((0, 1)));
}


#[test]
fn test_2() {
    let data_in = vec![3, 2, 4];
    let expected = Some((1, 2));
    assert_eq!(S::solve((data_in, 6)), expected);
}

#[test]
fn test_3() {
    let data_in = vec![3, 3];
    let expected = Some((0, 1));
    assert_eq!(S::solve((data_in, 6)), expected);
}