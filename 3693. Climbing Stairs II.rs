impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; n as usize + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            for j in i.saturating_sub(3)..i {
                dp[i] = dp[i].min(dp[j] + costs[i - 1] + ((i - j) * (i - j)) as i32);
            }
        }

        dp[dp.len() - 1]
    }
}
