use std::collections::{VecDeque, BinaryHeap};

impl Solution {
    pub fn maximum_safeness_factor(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let in_bound = |x: usize, y: usize| { x < n && y < n };
        let adjacent = |x: usize, y: usize| {
            [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)]
        };

        let mut queue = VecDeque::new();

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == 1 {
                    queue.push_back((x, y, 1));
                    grid[x][y] = 0;
                } else {
                    grid[x][y] = -1;
                }
            }
        }

        while let Some((x, y, safety)) = queue.pop_front() {
            for (new_x, new_y) in adjacent(x, y) {
                if !in_bound(new_x, new_y) || grid[new_x][new_y] != - 1 {
                    continue; 
                }

                grid[new_x][new_y] = safety;
                queue.push_back((new_x, new_y, safety + 1));
            }
        }

        let mut heap = BinaryHeap::new();
        heap.push((grid[0][0], 0, 0));

        while let Some((safety, x, y)) = heap.pop() {
            if x == n - 1 && y == n - 1 {
                return safety;
            }

            for (new_x, new_y) in adjacent(x, y) {
                if !in_bound(new_x, new_y) || grid[new_x][new_y] == -1 {
                    continue;
                }

                heap.push((safety.min(grid[new_x][new_y]), new_x, new_y));
                grid[new_x][new_y] = -1;
            }
        }

        0
    }
}
