impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let (mut rows, mut cols, mut servers) = (vec![0; grid.len()], vec![0; grid[0].len()], 0);

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                servers += i32::from(grid[i][j] == 1 && (rows[i] > 1 || cols[j] > 1));
            }
        }

        servers
    }
}
