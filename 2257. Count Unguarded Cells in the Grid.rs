impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);

        let mut grid  = vec![vec![0; n]; m];
        // We have at most m * n - len(walls) unguarded cells
        let mut count = m * n - walls.len();

        // We represent every cell as bitmask with
        // 111: Wall
        // 100: Guard
        // 010: Guarded with the respective guard being on the same row
        // 001: Guarded with the respective guard being on the same column

        // By using these masks, we can skip searching in one direction 
        // if another guard already covers the given row/column

        for wall in walls {
            let (row, col) = (wall[0] as usize, wall[1] as usize);
            grid[row][col] = 0b111;
        }

        for guard in guards {
            let (row, col) = (guard[0] as usize, guard[1] as usize);
            grid[row][col] ^= 0b100;

            let mut iter = (row..m);
            while let Some(i) = iter.next() && grid[i][col] & 0b001 == 0 {
                grid[i][col] ^= 0b001; 
                count -= ((grid[i][col] >> 1) & 1) ^ 1;
            }

            let mut iter = (0..row).rev();
            while let Some(i) = iter.next() && grid[i][col] & 0b001 == 0 {
                grid[i][col] ^= 0b001; 
                count -= ((grid[i][col] >> 1) & 1) ^ 1;
            }

            let mut iter = (col..n);
            while let Some(j) = iter.next() && grid[row][j] & 0b010 == 0 {
                grid[row][j] ^= 0b010; 
                count -= (grid[row][j] & 1) ^ 1;
            }

            let mut iter = (0..col).rev();
            while let Some(j) = iter.next() && grid[row][j] & 0b010 == 0 {
                grid[row][j] ^= 0b010; 
                count -= (grid[row][j] & 1) ^ 1;
            }
       }

        count as i32
    }
}
