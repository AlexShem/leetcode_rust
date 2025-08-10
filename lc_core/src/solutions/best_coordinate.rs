use super::Solve;
pub struct BestCoordinate;

impl BestCoordinate {
    pub fn best_coordinate(towers: Vec<Vec<i32>>, radius: i32) -> Vec<i32> {
        // towers[i] == [x_i, y_i, q_i]
        let n = towers.len();
        let scores: Vec<i32> = towers
            .iter()
            .map(|tower| {
                let mut signal = 0;
                for i in 0..n {
                    if let Some(quality) = Self::signal_quality(tower, &towers[i], &radius) {
                        signal += quality;
                    }
                }
                signal
            })
            .collect();
        let mut best_tower_score = scores[0];
        let mut best_tower = &towers[0];
        for i in 1..towers.len() {
            if scores[i] > best_tower_score {
                best_tower_score = scores[i];
                best_tower = &towers[i];
            } else if scores[i] == best_tower_score
                && (&towers[i][0] < &best_tower[0]
                || (&towers[i][0] < &best_tower[0] && &towers[i][1] < &best_tower[1]))
            {
                best_tower_score = scores[i];
                best_tower = &towers[i];
            }
        }

        if best_tower_score == 0 {
            return vec![0, 0];
        }

        vec![best_tower[0], best_tower[1]]
    }
    fn signal_quality(origin: &Vec<i32>, other: &Vec<i32>, radius: &i32) -> Option<i32> {
        let distance_2 = (origin[0] - other[0]).pow(2) + (origin[1] - other[1]).pow(2);
        if distance_2 < radius * radius {
            let quality = other[2] as f64 / (1.0 + (distance_2 as f64).sqrt());
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
