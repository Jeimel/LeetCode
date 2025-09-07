use std::iter::once;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        // We know that 1 + ... + n = n * (n + 1) / 2.
        // So, we add 1, ..., n -1 to the array, and 
        // in the end push their sum according to the 
        // gauss summation formula, which results in
        // n numbers where the last is the negated sum
        // of the first n - 1 numbers
        (1..n).chain(once(-(n * (n - 1) / 2))).collect()
    }
}
