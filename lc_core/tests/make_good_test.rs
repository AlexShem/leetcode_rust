use lc_core::solutions::{Solve, make_good::MakeGood as S};

#[test]
fn test_1() {
    let input = String::from("leEeetcode");
    let expected = String::from("leetcode");
    let result = S::solve(input);
    assert_eq!(result, expected);
}

#[test]
fn test_2() {
    let input = String::from("abBAcC");
    let expected = String::from("");
    let result = S::solve(input);
    assert_eq!(result, expected);
}

#[test]
fn test_3() {
    let input = String::from("s");
    let expected = String::from("s");
    let result = S::solve(input);
    assert_eq!(result, expected);
}

#[test]
fn test_4() {
    let input = String::from("Pp");
    let expected = String::from("");
    let result = S::solve(input);
    assert_eq!(result, expected);
}