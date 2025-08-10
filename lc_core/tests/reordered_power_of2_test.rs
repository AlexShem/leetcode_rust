use lc_core::solutions::Solve;
use lc_core::solutions::reordered_power_of2::ReorderedPowerOf2 as S;

#[test]
fn reordered_power_of2_lc1() {
    let input = 1;
    let result = S::solve(input);
    let expected = true;

    assert_eq!(result, expected)
}

#[test]
fn reordered_power_of2_lc2() {
    let input = 10;
    let result = S::solve(input);
    let expected = false;

    assert_eq!(result, expected)
}

#[test]
fn reordered_power_of2_lc3() {
    let input = 281;
    let result = S::solve(input);
    let expected = true;

    assert_eq!(result, expected)
}

#[test]
fn reordered_power_of2_lc4() {
    let input = 251;
    let result = S::solve(input);
    let expected = true;

    assert_eq!(result, expected)
}

#[test]
fn reordered_power_of2_lc5() {
    let input = 1_073_741_824;
    let result = S::solve(input);
    let expected = true;

    assert_eq!(result, expected)
}
