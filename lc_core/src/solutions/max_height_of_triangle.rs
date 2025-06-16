use super::Solve;

/// # 3200. Maximum Height of a Triangle
/// You are given two integers `red` and `blue` representing the count of red and blue colored balls.
/// You have to arrange these balls to form a triangle such that the 1st row will have
/// 1 ball, the 2nd row will have 2 balls, the 3rd row will have 3 balls, and so on.
///
/// All the balls in a particular row should be the **same** color,
/// and adjacent rows should have **different** colors.
///
/// Return the **maximum** height of the triangle that can be achieved.
pub struct MaxHeightOfTriangle;

impl MaxHeightOfTriangle {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        Self::construct_triangle(red, blue)
            .max(Self::construct_triangle(blue, red))
    }

    /// Determine the max height of a triangle
    /// with the first row being `num_1`.
    fn construct_triangle(mut num_1: i32, mut num_2: i32) -> i32 {
        let mut height = 0;
        for i in 1.. {
            if i & 1 == 1 {
                num_1 -= i;
            } else {
                num_2 -= i;
            }
            if num_1 < 0 || num_2 < 0 {
                break;
            }
            height += 1;
        }
        height
    }
}

impl Solve<(i32, i32), i32> for MaxHeightOfTriangle {
    fn solve(input: (i32, i32)) -> i32 {
        Self::max_height_of_triangle(input.0, input.1)
    }
}