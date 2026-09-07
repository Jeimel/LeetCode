impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (s, t) = (s.as_bytes(), t.as_bytes());

        let mut dp = vec![0; t.len() + 1];
        dp[t.len()] = 1;

        for i in (0..s.len()).rev() {
            for j in 0..t.len() {
                if s[i] == t[j] {
                    dp[j] += dp[j + 1];
                }
            }
        }

        println!("{dp:?}");

        dp[0]
    }
}
