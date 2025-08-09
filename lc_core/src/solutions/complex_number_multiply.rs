use super::Solve;

pub struct ComplexNumberMultiply;

impl ComplexNumberMultiply {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let (a, b) = num1[0..num1.len() - 1].split_once('+').unwrap();
        let (c, d) = num2[0..num2.len() - 1].split_once('+').unwrap();

        let a = a.parse::<i32>().unwrap();
        let b = b.parse::<i32>().unwrap();
        let c = c.parse::<i32>().unwrap();
        let d = d.parse::<i32>().unwrap();

        let real = a * c - b * d;
        let imag = b * c + a * d;

        format!("{}+{}i", real, imag)
    }
}

impl Solve<(String, String), String> for ComplexNumberMultiply {
    fn solve(input: (String, String)) -> String {
        Self::complex_number_multiply(input.0, input.1)
    }
}
