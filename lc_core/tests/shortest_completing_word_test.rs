use lc_core::solutions::Solve;
use lc_core::solutions::shortest_completing_word::ShortestCompletingWord as S;

#[test]
fn shortest_completing_word_lc1() {
    let license_plate = "1s3 PSt".to_string();
    let words: Vec<String> = vec!["step", "steps", "stripe", "stepple"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let expected = "steps".to_string();
    let result = S::solve((license_plate, words));
    assert_eq!(result, expected)
}

#[test]
fn shortest_completing_word_lc2() {
    let license_plate = "1s3 456".to_string();
    let words: Vec<String> = vec!["looks", "pest", "stew", "show"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let expected = "pest".to_string();
    let result = S::solve((license_plate, words));
    assert_eq!(result, expected)
}

#[test]
fn shortest_completing_word_lc11() {
    let license_plate = "Ah71752".to_string();
    let words: Vec<String> = vec!["suggest", "letter", "of", "husband", "easy", "education", "drug", "prevent", "writer", "old"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let expected = "husband".to_string();
    let result = S::solve((license_plate, words));
    assert_eq!(result, expected)
}