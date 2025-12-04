use std::collections::HashMap;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MOD: usize = 100_000_0007;

        points
            .iter()
            // We count number of points on the same horizontal line
            .fold(HashMap::new(), |mut acc, point| {
                *acc.entry(point[1]).or_insert(0usize) += 1; 
                acc
            }) 
            .into_values()
            // We always need two points to form a trapezoids
            .filter(|&v| v > 1)
            // We need the number of distinct 2-element combinations, which is equal to
            // v! / (2! * (v - 2)!) = (v * (v - 1)) / 2
            .map(|v| v * (v - 1) / 2)
            // We add up the number of combinations (`s`), and combine 
            // all previous combinations (`r`) with the current (`v`)
            .fold((0, 0), |(r, s), v| ((r + v * s) % MOD, s + v))
            .0 as i32
    }
}
