use lc_core::solutions::{Solve, max_height_of_triangle::MaxHeightOfTriangle as S};

#[test]
fn test_1() {
    let red = 2;
    let blue = 4;
    let expected = 3;
    let result = S::solve((red, blue));
    assert_eq!(result, expected)
}

#[test]
fn test_2() {
    let red = 2;
    let blue = 1;
    let expected = 2;
    let result = S::solve((red, blue));
    assert_eq!(result, expected)
}

#[test]
fn test_3() {
    let red = 1;
    let blue = 1;
    let expected = 1;
    let result = S::solve((red, blue));
    assert_eq!(result, expected)
}

#[test]
fn test_4() {
    let red = 10;
    let blue = 1;
    let expected = 2;
    let result = S::solve((red, blue));
    assert_eq!(result, expected)
}