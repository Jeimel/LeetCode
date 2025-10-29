impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        // We find the index after the most significant set bit,
        // which after a shift represents the smallest power of 2 
        // greater than `n`. Subtracting 1 from that gives a number,
        // where all bits below the power are set to 1
        (1 << (i32::BITS - n.leading_zeros()) as i32) - 1
    }
}
