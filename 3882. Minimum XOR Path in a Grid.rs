impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![[false; 1024]; grid[0].len()]; grid.len()];
        dp[0][0][grid[0][0] as usize] = true;

        let mut xor = grid[0][0] as usize;

        for i in 1..grid.len() {
            xor = xor ^ grid[i][0] as usize;
            dp[i][0][xor] = true;
        }

        xor = grid[0][0] as usize;

        for i in 1..grid[0].len() {
            xor = xor ^ grid[0][i] as usize;
            dp[0][i][xor] = true;
        }

        for i in 1..grid.len() {
            for j in 1..grid[i].len() {
                for k in 0..1024 {
                    if dp[i - 1][j][k] {
                        dp[i][j][k ^ grid[i][j] as usize] = true;
                    }

                    if dp[i][j - 1][k] {
                        dp[i][j][k ^ grid[i][j] as usize] = true;
                    }
                }
            }
        }

        for k in 0..1024 {
            if dp[grid.len() - 1][grid[0].len() - 1][k] {
                return k as i32;
            }
        }

        0        
    }
}
