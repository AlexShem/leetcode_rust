use lc_core::solutions::Solve;
use lc_core::solutions::largest_good_integer::LargestGoodInteger as S;

#[test]
fn largest_good_integer_lc1() {
    let num = "6777133339".to_string();
    let result = S::solve(num);
    let expected = "777".to_string();
    assert_eq!(result, expected)
}

#[test]
fn largest_good_integer_lc2() {
    let num = "2300019".to_string();
    let result = S::solve(num);
    let expected = "000".to_string();
    assert_eq!(result, expected)
}

#[test]
fn largest_good_integer_lc3() {
    let num = "42352338".to_string();
    let result = S::solve(num);
    let expected = "".to_string();
    assert_eq!(result, expected)
}