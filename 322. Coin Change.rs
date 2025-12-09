  impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;

        let mut dp = vec![i32::MAX; amount + 1];
        dp[0] = 0;

        for coin in coins {
            let coin = coin as usize;

            // We check if we can find any shorter sequence using `coin`
            for i in coin..=amount {
                // There exists no sequence in which `coin` can reach index `i`
                if dp[i - coin] == i32::MAX {
                    continue;
                }

                // Check if we found a short sequence
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }

        if dp[amount] != i32::MAX { dp[amount] } else { -1 }
    }
}
