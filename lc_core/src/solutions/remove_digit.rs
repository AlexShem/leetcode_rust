use super::Solve;

pub struct RemoveDigit;

impl RemoveDigit {
    // 133235:
    // 13235
    // 13235
    // 13325
    pub fn remove_digit(number: String, digit: char) -> String {
        let digit = digit as u8;
        let mut num = number.clone();

        // Special case: digit == '1'
        if digit == b'1' {
            let position = num.bytes().position(|i| i == digit).unwrap();
            num.remove(position);
            return num;
        }

        // Special case: digit == '9'
        if digit == b'9' {
            let position = num.bytes().rposition(|i| i == digit).unwrap();
            num.remove(position);
            return num;
        }

        let mut to_remove = None;
        for (idx, d) in num.bytes().enumerate() {
            if d == digit {
                // Check if the next digit exists and is greater
                if idx + 1 < num.len() && num.bytes().nth(idx + 1).unwrap() > d {
                    to_remove = Some(idx);
                    break; // Exit early when a valid digit is found
                }
                to_remove = Some(idx); // Keep track of the last found digit
            }
        }

        // Remove the selected digit
        num.remove(to_remove.unwrap());
        num
    }
}

impl Solve<(String, char), String> for RemoveDigit {
    fn solve(input: (String, char)) -> String {
        Self::remove_digit(input.0, input.1)
    }
}
