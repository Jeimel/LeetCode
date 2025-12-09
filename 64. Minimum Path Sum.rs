impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        dp[0][0] = grid[0][0];

        // We populate the first column with the cumulative sum
        for i in 1..grid.len() {
            dp[i][0] = dp[i - 1][0] + grid[i][0]
        }

        // We populate the first row with the cumulative sum
        for i in 1..grid[0].len() {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }

        for y in 1..grid.len() {
            for x in 1..grid[y].len() {
                // Can reach the current cell faster from the left or upper cell?
                dp[y][x] = grid[y][x] + dp[y - 1][x].min(dp[y][x - 1]);
            }
        }

        dp[grid.len() - 1][grid[0].len() - 1]
    }
}
