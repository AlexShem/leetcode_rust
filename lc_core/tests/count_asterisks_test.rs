use lc_core::solutions::{Solve, count_asterisks::CountAsterisks as S};

#[test]
fn test_1() {
    let input = String::from("l|*e*et|c**o|*de|");
    let expected = 2;
    let result = S::solve(input);
    assert_eq!(result, expected)
}

#[test]
fn test_2() {
    let input = String::from("iamprogrammer");
    let expected = 0;
    let result = S::solve(input);
    assert_eq!(result, expected)
}

#[test]
fn test_3() {
    let input = String::from("yo|uar|e**|b|e***au|tifu|l");
    let expected = 5;
    let result = S::solve(input);
    assert_eq!(result, expected)
}