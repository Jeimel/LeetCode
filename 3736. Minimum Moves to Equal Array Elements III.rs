impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let (sum, max) = nums
            .iter()
            .fold((0, 0), |(sum, max), &num| (sum + num, max.max(num)));

        max * nums.len() as i32 - sum
    }
}
