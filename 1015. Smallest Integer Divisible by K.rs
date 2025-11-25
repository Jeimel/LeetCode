impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        // We use 1 % k, in case k equals 1
        let (mut n, mut length) = (1 % k, 1);

        // If n == 0, then n % k = 0, and we found our solution
        while n != 0 {
            // There are at most `k` numbers in the remainder group of `k`
            if length == k {
                return -1;
            }

            // We can take our number modulo `k` in each iteration,
            // where we shift our number and append a new one
            n = (n * 10 + 1) % k;
            length += 1;
        }

        length
    }
}
