use lc_core::solutions::Solve;
use lc_core::solutions::int_to_roman::IntToRoman as S;

#[test]
fn int_to_roman_lc1() {
    let input = 3749;
    let result = S::solve(input);
    let expected = "MMMDCCXLIX".to_string();
    assert_eq!(result, expected)
}

#[test]
fn int_to_roman_lc2() {
    let input = 58;
    let result = S::solve(input);
    let expected = "LVIII".to_string();
    assert_eq!(result, expected)
}

#[test]
fn int_to_roman_lc3() {
    let input = 1994;
    let result = S::solve(input);
    let expected = "MCMXCIV".to_string();
    assert_eq!(result, expected)
}
