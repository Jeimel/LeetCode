impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;

        let mut dp = vec![vec![]; n + 1];

        dp[0].push(String::new());

        for i in 1..=n {
            for j in 0..i {
                for k in 0..dp[j].len() {
                    for l in 0..dp[i - j - 1].len() {
                        let mut s = String::with_capacity(2 * i);

                        // We built every combination of '(' + A ')' + B,
                        // where A and B are both valid with `i` pairs
                        s.push('(');
                        s.push_str(&dp[j][k]);
                        s.push(')');
                        s.push_str(&dp[i - j - 1][l]);

                        dp[i].push(s);
                    }
                }
            }
        }

        dp[n].clone()
    }
}
