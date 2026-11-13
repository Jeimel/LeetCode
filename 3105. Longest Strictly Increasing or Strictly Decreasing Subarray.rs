impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let (mut increasing, mut decreasing, mut max) = (1, 1, 1);

        for i in 1..nums.len() {
            increasing = increasing * i32::from(nums[i - 1] < nums[i]) + 1;
            decreasing = decreasing * i32::from(nums[i - 1] > nums[i]) + 1;

            max = max.max(increasing).max(decreasing);
        }

        max
    }
}
