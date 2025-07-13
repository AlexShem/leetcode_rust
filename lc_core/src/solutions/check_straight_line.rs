use super::Solve;

/// # 1232. Check If It Is a Straight Line
///
/// Easy
///
/// You are given an array coordinates, `coordinates[i] = [x, y]`, where `[x, y]` represents the coordinate of a point. Check if these points make a straight line in the XY plane.
pub struct CheckStraightLine;

impl CheckStraightLine {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let point_1 = &coordinates[0];
        let point_2 = &coordinates[1];

        let vec_1 = vec![point_2[0] - point_1[0], point_2[1] - point_1[1]];
        let result = true;

        for point in coordinates[2..].iter() {
            let vec_2 = vec![point[0] - point_1[0], point[1] - point_1[1]];
            let cross_product = vec_1[0] * vec_2[1] - vec_1[1] * vec_2[0];
            if cross_product != 0 {
                return false;
            }
        }

        result
    }
}

impl Solve<Vec<Vec<i32>>, bool> for CheckStraightLine {
    fn solve(input: Vec<Vec<i32>>) -> bool {
        Self::check_straight_line(input)
    }
}
