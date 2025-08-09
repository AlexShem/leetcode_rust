use lc_core::solutions::Solve;
use lc_core::solutions::is_power_of_two::IsPowerOfTwo as S;

#[test]
fn is_power_of_two_lc1() {
    let input = 1;
    let output = S::solve(input);

    assert!(output)
}

#[test]
fn is_power_of_two_lc2() {
    let input = 16;
    let output = S::solve(input);

    assert!(output)
}

#[test]
fn is_power_of_two_lc3() {
    let input = 3;
    let output = S::solve(input);

    assert!(!output)
}

#[test]
fn is_power_of_two_lc4() {
    let input = 0;
    let output = S::solve(input);

    assert!(!output)
}

#[test]
fn is_power_of_two_lc5() {
    let input = -16;
    let output = S::solve(input);

    assert!(!output)
}

#[test]
fn is_power_of_two_lc6() {
    let input = 21;
    let output = S::solve(input);

    assert!(!output)
}
