use super::Solve;

pub struct NearestValidPoint;

impl NearestValidPoint {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min_distance = i32::MAX;
        let mut min_index: i32 = -1;

        for (i, point) in points.iter().enumerate() {
            if x == point[0] || y == point[1] {
                let distance: i32 = (x.abs_diff(point[0]) + y.abs_diff(point[1])) as i32;
                if distance.lt(&min_distance) {
                    min_distance = distance;
                    min_index = i as i32;
                }
            }
        }
        min_index
    }
}

impl Solve<(i32, i32, Vec<Vec<i32>>), i32> for NearestValidPoint {
    fn solve(input: (i32, i32, Vec<Vec<i32>>)) -> i32 {
        Self::nearest_valid_point(input.0, input.1, input.2)
    }
}