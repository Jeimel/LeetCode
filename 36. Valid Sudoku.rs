impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool { 
        const SIZE: usize = 9;

        // Store each constraint inside a bit set, which
        // is indexed by id and digit
        let (mut rows, mut cols, mut sub_boxes) = (0u128, 0u128, 0u128);

        for i in 0..SIZE {
            for j in 0..SIZE {
                if !board[i][j].is_ascii_digit() {
                    continue;
                }

                // Convert char to integer starting at 0
                let digit = (board[i][j] as u8 - b'1') as usize;

                // Calculate indices for bit masks
                let row  = i * SIZE + digit;
                let col = j * SIZE + digit;
                // In math 3 * (i / 3) would equal i, due to integer division
                // this is not the case here.
                let sub_box = (3 * (i / 3) + j / 3) * SIZE + digit;

                // Check if any bit is already set. The results are combined and 
                // then checked to reduce clumsy if-statement
                if rows & (1 << row) | cols & (1 << col) | sub_boxes & (1 << sub_box) != 0 {
                    return false;
                }

                // Add each (index, digit) pair to the set
                rows |= 1 << row;
                cols |= 1 << col;
                sub_boxes |= 1 << sub_box;
            }
        }

        true
    }
}
