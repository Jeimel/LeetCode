impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter().fold(0, |acc, row| {
            acc + row.len() - row.partition_point(|x| *x >= 0)
        }) as i32
    }
}
