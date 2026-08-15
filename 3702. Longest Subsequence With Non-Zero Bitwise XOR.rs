impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let (mut xor, mut zero) = (0, true);

        for &num in &nums {
            xor ^= num;
            zero = zero && num == 0;
        }

        (nums.len()
            - if zero {
                nums.len()
            } else {
                usize::from(xor == 0)
            }) as i32
    }
}
