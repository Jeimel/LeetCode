impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        // The answer must be returned in modulo 10^9+7
        const MOD: i32 = 1000000007;

        let (n, delay, forget) = (n as usize, delay as usize, forget as usize);

        // Total stores how many people are currently aware of the secret, and
        // dp stores the amount of people being aware at day i
        let (mut total, mut dp) = (0, vec![0; n]);

        // One person discovers the secret on the first day
        dp[0] = 1;

        for i in delay..n {
            // We add the amount of people who will share the secret,
            // as they waited delay days 
            if i >= delay {
                total = (total + dp[i - delay]) % MOD;
            }

            // Store the amount of people being aware at the current day
            dp[i] = total;

            // We subtract the amount of people who will forget 
            // the secret after this day. That's why they are still
            // included in the total for the day
            if i + 1 >= forget {
                // We have to add MOD to make sure that the sum is positive
                total = (total - dp[i - forget + 1] + MOD) % MOD;
            }
        }

        // We only have to include the last forget days,
        // as everyone before has already forgotten the secret.
        dp[n - forget..]
            .iter()
            .fold(0, |total, &people| (total + people) % MOD)
    }
}
