impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        std::iter::once(".".repeat(n as usize))
            .chain((0..(m - 1)).map(|i| "#".repeat(n as usize - 1) + "."))
            .collect()
    }
}
