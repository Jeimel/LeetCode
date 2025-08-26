impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1];
        dp[1] = nums[0];

        // We can either rob house i and house i - 2, or house i - 1
        for i in 2..dp.len() {
            // dp[i] stores the maximum amount of money we can take from
            // all houses with index smaller than i.
            // len(nums) = len(dp) - 1, therefore we need to offset nums by -1
            dp[i] = (dp[i - 2] + nums[i - 1]).max(dp[i - 1]);
        }

        dp[nums.len()]
    }
}
