use lc_core::solutions::Solve;
use lc_core::solutions::decode_ciphertext::DecodeCiphertext as S;

#[test]
fn decode_ciphertext_lc1() {
    let encoded_text = String::from("ch   ie   pr");
    let rows = 3;
    let result = S::solve((encoded_text, rows));
    let expected = String::from("cipher");

    assert_eq!(result, expected)
}

#[test]
fn decode_ciphertext_lc2() {
    let encoded_text = String::from("iveo    eed   l te   olc");
    let rows = 4;
    let result = S::solve((encoded_text, rows));
    let expected = String::from("i love leetcode");

    assert_eq!(result, expected)
}

#[test]
fn decode_ciphertext_lc3() {
    let encoded_text = String::from("coding");
    let rows = 1;
    let result = S::solve((encoded_text, rows));
    let expected = String::from("coding");

    assert_eq!(result, expected)
}

#[test]
fn decode_ciphertext_lc4() {
    let encoded_text = String::from(" b  ac");
    let rows = 2;
    let result = S::solve((encoded_text, rows));
    let expected = String::from(" abc");

    assert_eq!(result, expected)
}
