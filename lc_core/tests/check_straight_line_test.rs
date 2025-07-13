use lc_core::solutions::Solve;
use lc_core::solutions::check_straight_line::CheckStraightLine as S;

#[test]
fn check_straight_line_lc1() {
    let coordinates = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]];
    let result = S::solve(coordinates);
    assert!(result)
}

#[test]
fn check_straight_line_lc2() {
    let coordinates = vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]];
    let result = S::solve(coordinates);
    assert!(!result)
}
