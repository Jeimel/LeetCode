impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        const PRIME: i32 = 0b1010_0010_1000_1010_1100;

        (left..=right).map(|n| PRIME >> n.count_ones() & 1).sum::<i32>()
    }
}
