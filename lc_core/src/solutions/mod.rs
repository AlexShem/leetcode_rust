pub mod two_sum;
pub mod maximize_array_sum;
pub mod is_palindrome;
pub mod is_ugly;
pub mod is_sum_equal;
pub mod minimum_average;
pub mod hamming_distance;
pub mod reverse_string;
pub mod xor_operation;
pub mod remove_digit;
pub mod merge_two_lists;
pub mod largest_integer;
pub mod count_asterisks;
pub mod nearest_valid_point;
pub mod make_good;
pub mod count_balls;
pub mod check_distances;
pub mod find_x_sum;
pub mod max_height_of_triangle;
pub mod pick_gifts;
pub mod is_possible_to_split;
pub mod bitset;
pub mod top_k_frequent;
pub mod min_end;
pub mod day_of_year;

pub trait Solve<I, O> {
    fn solve(input: I) -> O;
}
