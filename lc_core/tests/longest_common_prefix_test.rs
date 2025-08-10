use lc_core::solutions::Solve;
use lc_core::solutions::longest_common_prefix::LongestCommonPrefix as S;

#[test]
fn longest_common_prefix_lc1() {
    let strs = ["flower", "flow", "flight"];
    let strings: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
    let result = S::solve(strings);
    let expected = "fl".to_string();

    assert_eq!(result, expected)
}

#[test]
fn longest_common_prefix_lc2() {
    let strs = ["dog", "racecar", "car"];
    let strings: Vec<String> = strs.iter().map(|s| s.to_string()).collect();
    let result = S::solve(strings);
    let expected = "".to_string();

    assert_eq!(result, expected)
}
