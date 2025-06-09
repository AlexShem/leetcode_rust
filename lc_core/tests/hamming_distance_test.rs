use lc_core::solutions::{Solve, hamming_distance::HammingDistance as S};

#[test]
fn test_1() {
    let x = 1;
    let y = 4;
    let expected = 2;
    assert_eq!(S::solve((x, y)), expected);
}

#[test]
fn test_2() {
    let x = 3;
    let y = 1;
    let expected = 1;
    assert_eq!(S::solve((x, y)), expected);
}