impl Solution {
    pub fn min_swaps(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut swaps = 0;

        for i in 1..grid.len() {
            // Find next row, where the last n - `i` cells are zero
            let Some(position) = grid
                .iter()
                .position(|row| row[i..].iter().all(|&cell| cell == 0))
            else {
                return -1;
            };

            // Just removing the index is easier and more efficient than
            // sorting with index comparison
            grid.remove(position);
            
            // We can only swap adjacent rows, so we must swap `position` rows
            // in order to make the grid valid
            swaps += position as i32;
        }

        swaps
    }
}
