use itertools::Itertools;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        digits
            .iter()
            .permutations(3)
            .filter(|p| *p[0] > 0 && p[2] % 2 == 0)
            .unique()
            .count() as i32
    }
}
