use lc_core::solutions::{Solve, top_k_frequent::TopKFrequent as S};

#[test]
fn leetcode_347_1() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let expected = vec![1, 2];
    let result = S::solve((nums, k));
    assert_eq!(result, expected);
}

#[test]
fn leetcode_347_2() {
    let nums = vec![1];
    let k = 1;
    let expected = vec![1];
    let result = S::solve((nums, k));
    assert_eq!(result, expected);
}