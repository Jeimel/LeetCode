impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];

        for i in 0..s.len() {
            dp[i][i] = 1;
        }

        let bytes = s.as_bytes();

        for len in 2..=s.len() {
            for i in 0..=(s.len() - len) {
                let j = i + len - 1;

                dp[i][j] = if bytes[i] == bytes[j] {
                    2 + dp[i + 1][j - 1]
                } else {
                    dp[i + 1][j].max(dp[i][j - 1])
                };
            }
        }

        dp[0][s.len() - 1]
    }
}
