use super::Solve;
use std::collections::HashMap;

pub struct ReorderedPowerOf2;

impl ReorderedPowerOf2 {
    pub fn reordered_power_of2(n: i32) -> bool {
        let n_digits = Self::number_digits(&n);
        let n_frequencies = Self::frequency_table(&n);

        let powers_of2: Vec<i32> = (0..31)
            .filter_map(|power| {
                let power_of_2 = 1 << power;
                if Self::number_digits(&power_of_2).len() == n_digits.len() {
                    return Some(power_of_2);
                }
                None
            })
            .collect();

        for number in powers_of2 {
            let frequencies = Self::frequency_table(&number);

            if frequencies.len() == n_frequencies.len() {
                let matched_frequencies: bool = n_frequencies.iter().all(|(digit, count)| {
                    if frequencies.contains_key(digit) && frequencies[digit] == *count {
                        return true;
                    }
                    false
                });
                if matched_frequencies {
                    return true;
                }
            }
        }

        false
    }

    /// Calculates the digits frequency table of the `number`
    fn frequency_table(number: &i32) -> HashMap<i32, i32> {
        let mut frequencies = HashMap::with_capacity(10);
        let digits = Self::number_digits(&number);
        for digit in digits {
            frequencies
                .entry(digit)
                .and_modify(|d| *d += 1)
                .or_insert(1);
        }
        frequencies
    }

    fn number_digits(number: &i32) -> Vec<i32> {
        let mut number = number.clone();
        let mut digits = Vec::new();
        while number > 0 {
            let digit = number % 10;
            digits.push(digit);
            number /= 10;
        }
        digits
    }
}

impl Solve<i32, bool> for ReorderedPowerOf2 {
    fn solve(input: i32) -> bool {
        Self::reordered_power_of2(input)
    }
}
