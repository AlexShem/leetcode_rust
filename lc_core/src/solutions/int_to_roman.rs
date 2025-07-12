use crate::solutions::Solve;

/// # 12. Integer to Roman
///
/// **Difficulty:** Medium
///
/// ---
///
/// Roman numerals are represented by seven different symbols:
///
/// | Symbol | Value |
/// |--------|-------|
/// | I      | 1     |
/// | V      | 5     |
/// | X      | 10    |
/// | L      | 50    |
/// | C      | 100   |
/// | D      | 500   |
/// | M      | 1000  |
///
/// Roman numerals are formed by combining these symbols according to these rules:
///
/// 1. Roman numerals are written from largest to smallest from left to right
/// 2. When a smaller value precedes a larger value, it represents subtraction
/// 3. Only specific subtractive combinations are valid: IV (4), IX (9), XL (40),
///    XC (90), CD (400), and CM (900)
/// 4. Symbols I, X, C, and M can be repeated up to three times in succession
/// 5. Symbols V, L, and D cannot be repeated
///
/// ## Problem
///
/// Given an integer, convert it to a Roman numeral.
///
/// ## Constraints
///
/// - `1 <= num <= 3999`
pub struct IntToRoman;
impl IntToRoman {
    pub fn int_to_roman(mut num: i32) -> String {
        const VALUES: &[(&str, i32)] = &[
            ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
            ("C", 100), ("XC", 90), ("L", 50), ("XL", 40),
            ("X", 10), ("IX", 9), ("V", 5), ("IV", 4),
            ("I", 1)
        ];

        let mut result = String::new();

        for &(symbol, value) in VALUES {
            while num >= value {
                result.push_str(symbol);
                num -= value;
            }
        }

        result
    }
}

impl Solve<i32, String> for IntToRoman {
    fn solve(input: i32) -> String {
        Self::int_to_roman(input)
    }
}