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
        Self::construct_triangle(red, blue, true)
            .max(Self::construct_triangle(red, blue, false))
    }

    /// Determine the max height of a triangle
    /// with the first row being **blue** if `row_blue` is `true`,
    /// and with the first row being **red** otherwise.
    fn construct_triangle(mut red: i32, mut blue: i32, mut row_blue: bool) -> i32 {
        let mut height = 0;
        loop {
            match row_blue {
                true => {
                    if height + 1 <= blue {
                        height += 1;
                        blue -= height;
                        row_blue = !row_blue
                    } else {
                        break;
                    }
                }
                false => {
                    if height + 1 <= red {
                        height += 1;
                        red -= height;
                        row_blue = !row_blue;
                    } else {
                        break;
                    }
                }
            }
        }
        height
    }
}

impl Solve<(i32, i32), i32> for MaxHeightOfTriangle {
    fn solve(input: (i32, i32)) -> i32 {
        Self::max_height_of_triangle(input.0, input.1)
    }
}