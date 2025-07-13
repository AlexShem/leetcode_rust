use lc_core::solutions::Solve;
use lc_core::solutions::sum_four_divisors::SumFourDivisors as S;

#[test]
fn sum_four_divisors_lc1() {
    let nums = vec![21, 4, 7];
    let result = S::solve(nums);
    let expected = 32;
    assert_eq!(result, expected)
}

#[test]
fn sum_four_divisors_lc2() {
    let nums = vec![21, 21];
    let result = S::solve(nums);
    let expected = 64;
    assert_eq!(result, expected)
}

#[test]
fn sum_four_divisors_lc3() {
    let nums = vec![1, 2, 3, 4, 5];
    let result = S::solve(nums);
    let expected = 0;
    assert_eq!(result, expected)
}
