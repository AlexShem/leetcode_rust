use lc_core::solutions::{Solve, xor_operation::XorOperation as S};

#[test]
fn test_1() {
    let n = 5;
    let start = 0;
    let expected = 8;
    assert_eq!(S::solve((n, start)), expected);
}

#[test]
fn test_2() {
    let n = 4;
    let start = 3;
    let expected = 8;
    assert_eq!(S::solve((n, start)), expected);
}