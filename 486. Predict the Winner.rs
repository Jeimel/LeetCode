impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let mut dp = vec![vec![0; nums.len()]; nums.len()];

        for i in 0..nums.len() {
            dp[i][i] = nums[i];
        }

        for len in 2..=nums.len() {
            for i in 0..=(nums.len() - len) {
                let j = i + len - 1;

                dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
            }
        }

        dp[0][nums.len() - 1] >= 0
    }
}
