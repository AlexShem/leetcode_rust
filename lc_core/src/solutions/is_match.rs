use super::Solve;

pub struct IsMatch;

impl IsMatch {
    pub fn is_match(s: String, p: String) -> bool {
        Self::match_dp(&s, &p)
    }

    fn match_dp(s: &str, p: &str) -> bool {
        let s_chars: Vec<char> = s.chars().collect();
        let pattern_chars: Vec<char> = p.chars().collect();

        let p_chars = Self::compress_stars(&pattern_chars);

        let s_len = s_chars.len();
        let p_len = p_chars.len();

        // DP table: dp[i][j] = can s[0..i] match p[0..j]
        let mut dp = vec![vec![false; p_len + 1]; s_len + 1];
        dp[0][0] = true; // Empty string matches empty pattern

        // Empty string matches the '*'
        for j in 1..=p_len {
            if p_chars[j - 1] == '*' {
                dp[0][j] = dp[0][j - 1];
            }
        }

        for i in 1..=s_len {
            for j in 1..=p_len {
                let s_char = s_chars[i - 1];
                let p_char = p_chars[j - 1];

                match p_char {
                    '*' => {
                        // '*' can match empty sequence or any character
                        dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                    }
                    '?' => {
                        // '?' matches any single character
                        dp[i][j] = dp[i - 1][j - 1];
                    }
                    c => {
                        // Literal characters should match exactly
                        dp[i][j] = dp[i - 1][j - 1] && s_char == c;
                    }
                }
            }
        }
        dp[s_len][p_len]
    }

    fn compress_stars(pattern: &[char]) -> Vec<char> {
        let mut compressed = Vec::new();
        let mut i = 0;

        while i < pattern.len() {
            if pattern[i] == '*' {
                compressed.push('*');
                while i < pattern.len() && pattern[i] == '*' {
                    i += 1;
                }
            } else {
                compressed.push(pattern[i]);
                i += 1;
            }
        }

        compressed
    }
}

impl Solve<(String, String), bool> for IsMatch {
    fn solve(input: (String, String)) -> bool {
        Self::is_match(input.0, input.1)
    }
}
