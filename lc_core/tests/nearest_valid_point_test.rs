use lc_core::solutions::{Solve, nearest_valid_point::NearestValidPoint as S};

#[test]
fn test_1() {
    let x = 3;
    let y = 4;
    let points: Vec<Vec<i32>> = vec![[1, 2].to_vec(), [3, 1].to_vec(), [2, 4].to_vec(), [2, 3].to_vec(), [4, 4].to_vec()];
    let expected = 2;
    let result = S::solve((x, y, points));
    assert_eq!(result, expected);
}

#[test]
fn test_2() {
    let x = 3;
    let y = 4;
    let points: Vec<Vec<i32>> = vec![[3, 4].to_vec()];
    let expected = 0;
    let result = S::solve((x, y, points));
    assert_eq!(result, expected);
}

#[test]
fn test_3() {
    let x = 3;
    let y = 4;
    let points: Vec<Vec<i32>> = vec![[2, 3].to_vec()];
    let expected = -1;
    let result = S::solve((x, y, points));
    assert_eq!(result, expected);
}