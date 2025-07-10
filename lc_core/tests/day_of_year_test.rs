use lc_core::solutions::day_of_year::{DayOfYear as S};
use lc_core::solutions::Solve;

#[test]
fn day_of_year_lc1() {
    let input = "2019-01-09".to_string();
    let expected = 9;
    let output = S::solve(input);
    assert_eq!(output, expected)
}

#[test]
fn day_of_year_lc2() {
    let input = "2019-02-10".to_string();
    let expected = 41;
    let output = S::solve(input);
    assert_eq!(output, expected)
}