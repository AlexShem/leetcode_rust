use lc_core::solutions::Solve;
use lc_core::solutions::judge_circle::JudgeCircle as S;

#[test]
fn judge_circle_lc1() {
    let moves = String::from("UD");
    let result = S::solve(moves);
    let expected = true;

    assert_eq!(result, expected);
}

#[test]
fn judge_circle_lc2() {
    let moves = String::from("LL");
    let result = S::solve(moves);
    let expected = false;

    assert_eq!(result, expected);
}
