use crate::solutions::Solve;

pub struct DayOfYear;
impl DayOfYear {
    pub fn day_of_year(date: String) -> i32 {
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let parts: Vec<i32> = date
            .split('-')
            .map(|s| s.parse().unwrap())
            .collect();
        let year = parts[0];
        let month = parts[1] as usize;
        let day = parts[2];
        let days_acc: i32 = (&days_in_month[0..(month - 1)]).iter().sum();
        let days_num = days_acc + day;
        if Self::is_leap_year(year) && month > 2 {
            days_num + 1
        } else {
            days_num
        }
    }

    fn is_leap_year(year: i32) -> bool {
        if year % 400 == 0 {
            true
        } else if year % 4 == 0 && year % 100 != 0 {
            true
        } else {
            false
        }
    }
}

impl Solve<String, i32> for DayOfYear {
    fn solve(input: String) -> i32 {
        Self::day_of_year(input)
    }
}