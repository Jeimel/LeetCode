impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let (mut i, mut one) = (0, true);

        while i < bits.len() {
            one = bits[i] == 0;
            i += if one { 1 } else { 2 };
        }
        
        one
    }
}
