impl Solution {
    pub fn reverse_bits(mut n: i32) -> i32 {
       let mut rev = 0;

        while n != 0 {
            rev |= 1 << (31 - n.trailing_zeros()); 
            n &= n - 1
        }

        rev 
    }
}
