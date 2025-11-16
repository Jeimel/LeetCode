impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: u64 = 1_000_000_007;

        let (mut count, mut result) = (0u64, 0u64);

        for b in s.bytes() {
            if b == b'0' {
                count = 0;
                continue;
            }
 
            count += 1;
            result += count;
        }

        (result % MOD) as i32
    }
}
