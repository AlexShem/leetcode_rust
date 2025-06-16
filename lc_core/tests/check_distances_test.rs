use lc_core::solutions::{Solve, check_distances::CheckDistances as S};

#[test]
fn test_1() {
    let s = String::from("abaccb");
    let distance = vec![1, 3, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let expected = true;
    let result = S::solve((s, distance));
    assert_eq!(result, expected)
}

#[test]
fn test_2() {
    let s = String::from("aa");
    let distance = vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let expected = false;
    let result = S::solve((s, distance));
    assert_eq!(result, expected)
}
