use super::Solve;
pub struct BestCoordinate;

impl BestCoordinate {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        // towers[i] == [x_i, y_i, q_i]
        let towers_x: Vec<i32> = towers.iter().map(|tower| tower[0]).collect();
        let towers_y: Vec<i32> = towers.iter().map(|tower| tower[1]).collect();
        let min_x = towers_x.iter().min().copied().unwrap();
        let min_y = towers_y.iter().min().copied().unwrap();
        let max_x = towers_x.iter().max().copied().unwrap();
        let max_y = towers_y.iter().max().copied().unwrap();

        let mut best_location = vec![0, 0];
        let mut best_signal = 0;

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let mut total_signal = 0;
                for tower in &towers {
                    if let Some(signal) = Self::signal_quality(&vec![x, y], tower, &radius) {
                        total_signal += signal;
                    }
                }
                if total_signal > best_signal
                    || (total_signal == best_signal
                        && (x < best_location[0]
                            || (x == best_location[0] && y < best_location[1])))
                {
                    best_signal = total_signal;
                    best_location = vec![x, y];
                }
            }
        }
        best_location
    }

    /// Calculates the signal quality from a given point to a target tower
    /// - `origin`: [x, y] coordinates of the point
    /// - `tower`: [tx, ty, q] coordinates of the tower and its signal quality `q`
    /// - `radius`: the radius at which the tower broadcasts
    fn signal_quality(origin: &Vec<i32>, tower: &Vec<i32>, radius: &i32) -> Option<i32> {
        let distance_2 = (origin[0] - tower[0]).pow(2) + (origin[1] - tower[1]).pow(2);
        if distance_2 <= radius * radius {
            let quality = tower[2] as f64 / (1.0 + (distance_2 as f64).sqrt());
            return Some(quality as i32);
        }
        None
    }
}

impl Solve<(Vec<Vec<i32>>, i32), Vec<i32>> for BestCoordinate {
    fn solve(input: (Vec<Vec<i32>>, i32)) -> Vec<i32> {
        Self::best_coordinate(input.0, input.1)
    }
}
