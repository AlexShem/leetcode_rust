use super::Solve;

pub struct RepeatLimitedString;

impl RepeatLimitedString {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut output = String::new();

        let mut frequencies = [0; 26];
        for char in s.chars() {
            let pos = (char as u8 - b'a') as usize;
            frequencies[pos] += 1;
        }

        loop {
            let mut right: i32 = 25;
            // Find the biggest letter that we can add
            while right >= 0 && frequencies[right as usize] == 0 {
                right -= 1;
            }
            if right < 0 {
                // No more letters to add
                break;
            }
            let mut rep = repeat_limit.min(frequencies[right as usize]);
            let ch = (b'a' + (right as u8)) as char;
            // If the last added character is the same, make sure it is added at most `repeat_limit` times
            if let Some(last) = output.chars().last() {
                if ch == last {
                    rep = rep.min(repeat_limit - 1)
                }
            }
            output.push_str(ch.to_string().as_str().repeat(rep as usize).as_str());
            frequencies[right as usize] -= rep;

            // Try adding one more smaller letter
            if right == 0 {
                break;
            } else {
                right -= 1;
                // Find the next (smaller letter)
                while right >= 0 && frequencies[right as usize] == 0 {
                    right -= 1;
                }
                if right < 0 {
                    // No more letters to add
                    break;
                }
                output.push((b'a' + (right as u8)) as char);
                frequencies[right as usize] -= 1;
            }
        }

        output
    }
}

impl Solve<(String, i32), String> for RepeatLimitedString {
    fn solve(input: (String, i32)) -> String {
        Self::repeat_limited_string(input.0, input.1)
    }
}