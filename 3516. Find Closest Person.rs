use std::cmp::Ordering;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        // Determine difference in absolute distance from x and y to z
        match (x - z).abs().cmp(&(y - z).abs()) {
            Ordering::Equal => 0,
            Ordering::Less => 1,
            Ordering::Greater => 2,
        }
    }
}
