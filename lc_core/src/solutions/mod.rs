pub mod two_sum;
pub mod maximize_array_sum;
pub mod is_palindrome;
pub mod is_ugly;
pub mod is_sum_equal;
pub mod minimum_average;
pub mod hamming_distance;
pub mod reverse_string;

pub trait Solve<I, O> {
    fn solve(input: I) -> O;
}
