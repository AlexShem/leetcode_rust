use lc_core::solutions::Solve;
use lc_core::solutions::best_coordinate::BestCoordinate as S;

#[test]
fn best_coordinate_lc1() {
    let towers = vec![vec![1, 2, 5], vec![2, 1, 7], vec![3, 1, 9]];
    let radius = 2;
    let result = S::solve((towers, radius));
    let expected = vec![2, 1];

    assert_eq!(result, expected)
}

#[test]
fn best_coordinate_lc2() {
    let towers = vec![vec![23, 11, 21]];
    let radius = 9;
    let result = S::solve((towers, radius));
    let expected = vec![23, 11];

    assert_eq!(result, expected)
}

#[test]
fn best_coordinate_lc3() {
    let towers = vec![vec![1, 2, 13], vec![2, 1, 7], vec![0, 1, 9]];
    let radius = 2;
    let result = S::solve((towers, radius));
    let expected = vec![1, 2];

    assert_eq!(result, expected)
}

#[test]
fn best_coordinate_lc4() {
    let towers = vec![vec![42, 0, 0]];
    let radius = 7;
    let result = S::solve((towers, radius));
    let expected = vec![0, 0];

    assert_eq!(result, expected)
}

#[test]
fn best_coordinate_lc5() {
    let towers = vec![vec![0, 1, 2], vec![2, 1, 2], vec![1, 0, 2], vec![1, 2, 2]];
    let radius = 1;
    let result = S::solve((towers, radius));
    let expected = vec![1, 1];

    assert_eq!(result, expected)
}