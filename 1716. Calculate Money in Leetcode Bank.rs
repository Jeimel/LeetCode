impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks = n / 7;
        let days = n % 7;

        28 * weeks
            + 7 * weeks * (weeks - 1) / 2 
            + (days + 1) * days / 2 
            + days * weeks
    }
}
