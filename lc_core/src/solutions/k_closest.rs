use super::Solve;
use std::collections::BinaryHeap;

pub struct KClosest;

impl KClosest {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut max_heap = BinaryHeap::with_capacity(points.len());
        for point in points {
            let square_distance = Self::distance_to_origin(&point);
            if max_heap.len() < k as usize {
                max_heap.push((square_distance, point));
            } else if let Some(&(max_dist, _)) = max_heap.peek() {
                if square_distance < max_dist {
                    max_heap.pop();
                    max_heap.push((square_distance, point));
                }
            }
        }

        max_heap.into_iter().map(|(_, point)| point).collect()
    }

    fn distance_to_origin(point: &Vec<i32>) -> i32 {
        point[0].pow(2) + point[1].pow(2)
    }
}

impl Solve<(Vec<Vec<i32>>, i32), Vec<Vec<i32>>> for KClosest {
    fn solve(input: (Vec<Vec<i32>>, i32)) -> Vec<Vec<i32>> {
        Self::k_closest(input.0, input.1)
    }
}
