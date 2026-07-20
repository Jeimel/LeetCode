impl Solution {
    pub fn shift_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let (k, m, n) = (k as usize, grid.len(), grid[0].len());
        let len = m * n;

        let mut shift = k % len;
        let mut start = 0;

        for _ in 0..len {
            let (old_i, old_j) = (start / n, start % n);
            let (new_i, new_j) = (shift / n, shift % n);

            (grid[old_i][old_j], grid[new_i][new_j]) = (grid[new_i][new_j], grid[old_i][old_j]); 

            shift = (shift + k) % len;
            if shift == start {
                shift += 1;
                start += 1;
            }
        }

        grid
    }
}
