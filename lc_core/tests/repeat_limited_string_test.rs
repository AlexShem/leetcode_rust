use lc_core::solutions::repeat_limited_string::RepeatLimitedString as S;
use lc_core::solutions::Solve;

#[test]
fn repeat_limited_string_lc1() {
    let input = String::from("cczazcc");
    let repeat_limit = 3;
    let result = S::solve((input, repeat_limit));
    let expected = String::from("zzcccac");

    assert_eq!(result, expected);
}

#[test]
fn repeat_limited_string_lc2() {
    let input = String::from("aababab");
    let repeat_limit = 2;
    let result = S::solve((input, repeat_limit));
    let expected = String::from("bbabaa");

    assert_eq!(result, expected);
}
