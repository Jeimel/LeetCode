impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        // `low` represents the first odd number greater or equal to the original `low`
        let low = low + (1 - low % 2);
        // `high` represents the first odd number smaller or equal to the original `high`
        let high = high - (1 - high % 2);

        // We can narrow the interval of [low, high], because even numbers are not counted anyways.
        // Now, both numbers are odd, and we can simply calculate odd numbers in their interval.
        (high - low) / 2 + 1
    }
}
