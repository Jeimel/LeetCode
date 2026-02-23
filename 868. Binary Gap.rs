impl Solution {
    pub fn binary_gap(mut n: i32) -> i32 {
        let (mut max, mut previous) = (0, 0);

        loop {
            previous = n.trailing_zeros();

            n &= n - 1;
            if n == 0 { break; }

            max = max.max(n.trailing_zeros() - previous);
        }

        max as i32
    }
}
