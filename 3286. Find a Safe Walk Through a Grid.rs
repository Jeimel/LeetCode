use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(mut grid: Vec<Vec<i32>>, health: i32) -> bool {
        let mut queue = VecDeque::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        queue.push_back((0, 0, grid[0][0]));
        visited[0][0] = true;

        while let Some((x, y, safe)) = queue.pop_front() {
            if x == grid.len() - 1 && y == grid[x].len() - 1 {
                return health - safe > 0;
            }

            if health - safe <= 0 {
                break;
            }

            for (new_x, new_y) in [
                (x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)
            ] {
                if new_x >= grid.len() || new_y >= grid[new_x].len() || visited[new_x][new_y] {
                    continue;
                }

                visited[new_x][new_y] = true;
                if grid[new_x][new_y] == 1 {
                    queue.push_back((new_x, new_y, safe + 1));
                } else {
                    queue.push_front((new_x, new_y, safe));
                }
            }
        }

        false
    }
}
