use lc_core::solutions::Solve;
use lc_core::solutions::k_closest::KClosest as S;

#[test]
fn k_closest_lc1() {
    let points = vec![vec![1, 3], vec![-2, 2]];
    let k = 1;
    let result = S::solve((points, k));
    let expected = vec![vec![-2, 2]];

    assert_eq!(result, expected)
}
