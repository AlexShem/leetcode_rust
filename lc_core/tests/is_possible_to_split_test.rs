use lc_core::solutions::{Solve, is_possible_to_split::IsPossibleToSplit as S};

#[test]
fn test_1() {
    let nums = vec![1, 1, 2, 2, 3, 4];
    let expected = true;
    let result = S::solve(nums);
    assert_eq!(result, expected)
}

#[test]
fn test_2() {
    let nums = vec![1, 1, 1, 1];
    let expected = false;
    let result = S::solve(nums);
    assert_eq!(result, expected)
}