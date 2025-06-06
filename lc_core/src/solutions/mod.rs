pub mod two_sum;

pub trait Solve<I, O> {
    fn solve(input: I) -> O;
}
