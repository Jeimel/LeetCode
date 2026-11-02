impl Solution {
    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let median = nums[(nums.len() - 1) / 2];

        nums.iter()
            .fold(0, |moves, num| moves + (num - median).abs())
    }
}
