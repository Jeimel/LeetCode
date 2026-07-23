impl Solution {
pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => 1,
            2 => 2,
            _ => 1 << 32 - (nums.len() as u32).leading_zeros()
        }
    }
}
