use lc_core::solutions::{Solve, reverse_string::ReverseString as S};

#[test]
fn test_1() {
    let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    let expected: Vec<char> = vec!['o', 'l', 'l', 'e', 'h'];
    S::solve(&mut s);
    assert_eq!(s, expected);
}

#[test]
fn test_2() {
    let mut s: Vec<char> = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    let expected: Vec<char> = vec!['h', 'a', 'n', 'n', 'a', 'H'];
    S::solve(&mut s);
    assert_eq!(s, expected);
}
