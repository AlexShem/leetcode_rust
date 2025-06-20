use lc_core::solutions::{Solve, pick_gifts::PickGifts as S};

#[test]
fn test_1() {
    let gifts = vec![25, 64, 9, 4, 100];
    let k = 4;
    let expected = 29;
    let result = S::solve((gifts, k));
    assert_eq!(result, expected);
}

#[test]
fn test_2() {
    let gifts = vec![1, 1, 1, 1];
    let k = 4;
    let expected = 4;
    let result = S::solve((gifts, k));
    assert_eq!(result, expected);
}