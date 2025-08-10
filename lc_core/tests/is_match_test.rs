use lc_core::solutions::Solve;
use lc_core::solutions::is_match::IsMatch as S;

#[test]
fn is_match_lc1() {
    let s = String::from("aa");
    let p = String::from("a");
    let result = S::solve((s, p));
    let expected = false;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc2() {
    let s = String::from("aa");
    let p = String::from("*");
    let result = S::solve((s, p));
    let expected = true;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc3() {
    let s = String::from("cb");
    let p = String::from("?a");
    let result = S::solve((s, p));
    let expected = false;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc4() {
    let s = String::from("abceb");
    let p = String::from("*a*b");
    let result = S::solve((s, p));
    let expected = true;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc5() {
    let s = String::from("adceb");
    let p = String::from("?a?b");
    let result = S::solve((s, p));
    let expected = false;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc6() {
    let s = String::from("a");
    let p = String::from("***");
    let result = S::solve((s, p));
    let expected = true;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc7() {
    let s = String::from("aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba");
    let p = String::from("a*******b");
    let result = S::solve((s, p));
    let expected = false;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc8() {
    let s = String::from("bbbababbbbabbbbababbaaabbaababbbaabbbaaaabbbaaaabb");
    let p = String::from("*b********bb*b*bbbbb*ba");
    let result = S::solve((s, p));
    let expected = false;

    assert_eq!(result, expected)
}

#[test]
fn is_match_lc9() {
    let s = String::from("azb");
    let p = String::from("a**b");
    let result = S::solve((s, p));
    let expected = true;

    assert_eq!(result, expected)
}
