impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        nums.iter().enumerate().fold(0, |sum, (i, num)| {
            sum + num * num * i32::from(nums.len() % (i + 1) == 0)
        })
    }
}
