use lc_core::solutions::Solve;
use lc_core::solutions::complex_number_multiply::ComplexNumberMultiply as S;

#[test]
fn complex_number_multiply_lc1() {
    let num1 = String::from("1+1i");
    let num2 = String::from("1+1i");
    let result = S::solve((num1, num2));
    let expected = String::from("0+2i");

    assert_eq!(result, expected)
}

#[test]
fn complex_number_multiply_lc2() {
    let num1 = String::from("1+-1i");
    let num2 = String::from("1+-1i");
    let result = S::solve((num1, num2));
    let expected = String::from("0+-2i");

    assert_eq!(result, expected)
}
