struct Spreadsheet {
    sheet: Vec<i32>,
}

impl Spreadsheet {
    fn new(rows: i32) -> Self {
        Self {
            sheet: vec![0; rows as usize * 26]
        } 
    }

    fn index(&self, cell: &str) -> usize {
        let col = (cell.bytes().next().unwrap() - b'A') as usize;
        // We have to subtact 1, as the first row is referenced by 1
        let row = cell[1..].parse::<usize>().unwrap() - 1;

        row * 26 + col
    }
    
    fn set_cell(&mut self, cell: String, value: i32) {
        // We must store the immutable call variable first
        let index = self.index(&cell);

        // Then we can mutate
        self.sheet[index] = value;
    }
    
    fn reset_cell(&mut self, cell: String) {
        // Just set the cell to zero
        self.set_cell(cell, 0);
    }
    
    fn get_value(&mut self, formula: String) -> i32 {
        // We remove the equal sign, and split once at the plus to get the two addends
        let (x, y) = formula[1..].split_once('+').unwrap();

        // If the first character is already an ascii digit, we don't have a
        // cell reference. So we parse the result, otherwise we take the cell value

        let x = if x.bytes().next().unwrap().is_ascii_digit() {
            x.parse().unwrap()
        } else {
            self.sheet[self.index(&x)]
        };

        let y = if y.bytes().next().unwrap().is_ascii_digit() {
            y.parse().unwrap()
        } else {
            self.sheet[self.index(&y)]
        };

        x + y
    }
}
