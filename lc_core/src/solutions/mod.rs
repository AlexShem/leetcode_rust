pub mod two_sum;
pub mod maximize_array_sum;

pub trait Solve<I, O> {
    fn solve(input: I) -> O;
}
