use lc_core::solutions::{Solve, largest_integer::LargestInteger as S};

#[test]
fn test_1() {
    let input = 1234;
    let expected = 3412;
    let result = S::solve(input);

    assert_eq!(result, expected);
}

#[test]
fn test_2() {
    let input = 65875;
    let expected = 87655;
    let result = S::solve(input);

    assert_eq!(result, expected);
}