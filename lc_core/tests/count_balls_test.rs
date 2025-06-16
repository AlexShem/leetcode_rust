use lc_core::solutions::{Solve, count_balls::CountBalls as S};

#[test]
fn test_1() {
    let low_limit = 1;
    let high_limit = 10;
    let expected = 2;
    let result = S::solve((low_limit, high_limit));
    assert_eq!(result, expected)
}

#[test]
fn test_2() {
    let low_limit = 5;
    let high_limit = 15;
    let expected = 2;
    let result = S::solve((low_limit, high_limit));
    assert_eq!(result, expected)
}

#[test]
fn test_3() {
    let low_limit = 19;
    let high_limit = 28;
    let expected = 2;
    let result = S::solve((low_limit, high_limit));
    assert_eq!(result, expected)
}