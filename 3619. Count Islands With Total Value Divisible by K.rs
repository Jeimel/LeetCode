impl Solution {
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (mut islands, mut stack, mut sum, k) = (0, vec![], 0, k as i64);

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    continue;
                }

                sum = 0;
                stack.push((i, j));
 
                while let Some((i, j)) = stack.pop() {
                    sum += grid[i][j] as i64;
                    grid[i][j] = 0;

                    if i > 0 && grid[i - 1][j] != 0 {
                        stack.push((i - 1, j));
                    }

                    if i + 1 < grid.len() && grid[i + 1][j] != 0 {
                        stack.push((i + 1, j));
                    }

                    if j > 0 && grid[i][j - 1] != 0 {
                        stack.push((i, j - 1));
                    }

                    if j + 1 < grid[i].len() && grid[i][j + 1] != 0 {
                        stack.push((i, j + 1));
                    }
                }

                islands += i32::from(sum % k == 0);
            }
        }

        islands 
    }
}
