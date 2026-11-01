impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        nums.iter().fold(
            -(nums.len() as i32 * nums.iter().min().unwrap()),
            |acc, num| acc + num,
        )
    }
}
