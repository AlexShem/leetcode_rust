// lc_core/tests/two_sum_test.rs
use lc_core::solutions::{Solve, two_sum::TwoSum as S};

#[test]
fn works_on_extra_cases() {
    let data_in = vec![3, 2, 4];
    let expected = Some((1, 2));
    assert_eq!(S::solve((data_in, 6)), expected);
}
