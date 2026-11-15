impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let (mut count, mut parity, mut even, mut odd) = (0i64, 0, 1i64, 0i64);

        for num in arr {
            parity ^= num & 1;

            (count, even, odd) = if parity == 0 {
                (count + odd, even + 1, odd)
            } else {
                (count + even, even, odd + 1)
            };
        }

        (count % 1_000_000_007) as i32
    }
}
