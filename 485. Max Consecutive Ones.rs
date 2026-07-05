impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        (0..nums.len()).fold((0, 0), |(max, count), i| {
            let count = if nums[i] == 1 {
                count + 1
            } else {
                0
            };

            (max.max(count), count)
        }).0
    }
}
