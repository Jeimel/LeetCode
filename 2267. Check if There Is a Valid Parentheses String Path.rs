impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let mut dp = vec![vec![1u128; grid[0].len()]; grid.len()];

        if grid[0][0] == ')' {
            return false;
        }

        dp[0][0] <<= 1;

        for i in 1..grid[0].len() {
            dp[0][i] = if grid[0][i] == '(' { 
                dp[0][i - 1] << 1
            } else {
                dp[0][i - 1] >> 1
            };
        }

        for i in 1..grid.len() {
            dp[i][0] = if grid[i][0] == '(' { 
                dp[i - 1][0] << 1
            } else {
                dp[i - 1][0] >> 1
            };
        }

        for i in 1..grid.len() {
            for j in 1..grid[i].len() {
                dp[i][j] = if grid[i][j] == '(' { 
                    (dp[i - 1][j] | dp[i][j - 1]) << 1
                } else {
                    (dp[i - 1][j] | dp[i][j - 1]) >> 1
                };
            }
        }

        (dp.last().unwrap().last().unwrap() & 1) == 1
    }
}
