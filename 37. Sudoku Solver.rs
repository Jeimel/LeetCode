impl Solution {
    const SIZE: usize = 9;

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let (mut rows, mut cols, mut sub_boxes) = (0u128, 0u128, 0u128);

        for i in 0..Self::SIZE {
            for j in 0..Self::SIZE {
                if !board[i][j].is_ascii_digit() {
                    continue;
                }

                // Convert char to integer starting at 0
                let digit = (board[i][j] as u8 - b'1') as usize;

                // Add each (index, digit) pair to the set
                rows |= 1 << (i * Self::SIZE + digit);
                cols |= 1 << (j * Self::SIZE + digit);
                sub_boxes |= 1 << ((3 * (i / 3) + j / 3) * Self::SIZE + digit);
            }
        }

        Self::solve(board, rows, cols, sub_boxes, 0, 0);
    }

    fn solve(
        board: &mut Vec<Vec<char>>,
        rows: u128,
        cols: u128,
        sub_boxes: u128,
        i: usize,
        j: usize,
    ) -> bool {
        // We have reached the end, so the current configuration is valid
        if i == Self::SIZE {
            return true;
        }

        // We reached the end of the row, so we reset the column and increase the row
        if j == Self::SIZE {
            return Self::solve(board, rows, cols, sub_boxes, i + 1, 0);
        }

        // Is the current cell already filled?
        // If yes, we skip this cell
        if board[i][j].is_ascii_digit() {
            return Self::solve(board, rows, cols, sub_boxes, i, j + 1);
        }

        // Shift the current row/col/sub-box to the front and negate the result 
        // to get all possible digits for the current cell
        let mut digits = 0b111111111
            & (!(rows >> i * Self::SIZE)
                & !(cols >> j * Self::SIZE)
                & !(sub_boxes >> (3 * (i / 3) + j / 3) * Self::SIZE));

        // Every digit in the set represents a valid configuration, 
        // as the digit bit is not set in any of the three constraints
        while digits != 0 {
            let digit = digits.trailing_zeros() as usize;
            board[i][j] = char::from_digit(digit as u32+ 1, 10).unwrap();

            // Can we solve the sudoku based on the current configuration?
            if Self::solve(
                board, 
                rows | (1 << i * Self::SIZE + digit), 
                cols | (1 << j * Self::SIZE + digit), 
                sub_boxes | (1 << (3 * (i / 3) + j / 3) * Self::SIZE + digit), 
                i, 
                j + 1
            ) {
                return true;
            }

            // We can't solve the sudoku using the digit at the current index
            board[i][j] = '.';

            // Step to next set bit
            digits &= digits.wrapping_sub(1);
        }

        // We have not found any solvable digit for the current index.
        // Therefore, we must step back and have to find a valid digit earlier
        false
    }
}
