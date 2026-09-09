impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        (3..)
            .step_by(3)
            .map(|i| (n - (10i64.pow(i) - 1)).max(0))
            .take_while(|&i| i != 0)
            .sum()
    }
}
