use lc_core::solutions::{Solve, min_end::MinEnd as S};

#[test]
fn min_end_lc1() {
    let n = 3;
    let x = 4;
    let expected = 6_i64;
    let result = S::solve((n, x));
    assert_eq!(result, expected)
}

#[test]
fn min_end_lc2() {
    let n = 2;
    let x = 7;
    let expected = 15_i64;
    let result = S::solve((n, x));
    assert_eq!(result, expected)
}

#[test]
fn min_end_lc709() {
    let n = 6715154;
    let x = 7193485;
    let expected = 55012476815_i64;
    let result = S::solve((n, x));
    assert_eq!(result, expected)
}