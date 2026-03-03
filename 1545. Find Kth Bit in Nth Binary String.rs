use std::cmp::Ordering;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        if n == 1 {
            return '0';
        }

        // S_n has a length of 2^n - 1
        let mid = 1 << (n - 1);

        match k.cmp(&mid) {
            // The middle will always be `1`
            Ordering::Equal => '1',
            // We know that S_{n - 1}1(...), so we can just continue searching in S_{n - 1}
            Ordering::Less => Self::find_kth_bit(n - 1, k),
            // We know that the `k`-th bit is reversed and flipped based on S_{n - 1},
            // so we just mirror `k` around the middle and flip the result
            Ordering::Greater => char::from(Self::find_kth_bit(n - 1, (mid << 1) - k) as u8 ^ 1),
        }
    }
}
