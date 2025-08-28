use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut decreasing = BinaryHeap::new();

        // We have decreasing oder if j <= i
        for i in 0..grid.len() {
            decreasing = (0..(grid.len() - i)).map(|j| grid[i + j][j]).collect();

            for j in 0..(grid.len() - i) {
                grid[i + j][j] = decreasing.pop().unwrap();
            }

            decreasing.clear();
        }

        let mut increasing = BinaryHeap::new();

        // We have increasing oder if j > i
        for j in 1..grid.len() {
            increasing = (0..(grid.len() - j)).map(|i| Reverse(grid[i][i + j])).collect();

            for i in 0..(grid.len() - j) {
                grid[i][j + i] = increasing.pop().unwrap().0;
            }

            increasing.clear();
        }

        grid
    }
}
