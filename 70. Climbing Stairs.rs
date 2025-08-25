impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // n depends on the number of ways to climb n - 1 and n - 2,
        // due to the possible number of steps per climb (1 or 2).
        // The base cases are 0 for n = 0 and 1 for n = 1.
        // In other words, we have to calculate the n-th fibbonacci number.
        (0..n)
            .fold((1, 0), |(current, previous), _| (current + previous, current))
            .0
    }
}
