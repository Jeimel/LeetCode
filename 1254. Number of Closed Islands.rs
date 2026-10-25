impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let (mut visited, mut islands) = (vec![vec![false; grid[0].len()]; grid.len()], 0);

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if visited[i][j] || grid[i][j] == 1 {
                    continue;
                }

                let (mut stack, mut valid) = (vec![(i, j)], true);
                
                while let Some((i, j)) = stack.pop() {
                    if visited[i][j] {
                        continue;
                    }

                    visited[i][j] = true;

                    if i == 0 || j == 0 || i == grid.len() - 1 || j == grid[i].len() - 1 {
                        valid = false;
                    }

                    if i > 0 && grid[i - 1][j] == 0 {
                        stack.push((i - 1, j));
                    }

                    if i + 1 < grid.len() && grid[i + 1][j] == 0 {
                        stack.push((i + 1, j));
                    }

                    if j > 0 && grid[i][j - 1] == 0 {
                        stack.push((i, j - 1));
                    }

                    if j + 1 < grid[i].len() && grid[i][j + 1] == 0 {
                        stack.push((i, j + 1));
                    }
                }

                islands += i32::from(valid);
            }
        }

        islands
    }
}
