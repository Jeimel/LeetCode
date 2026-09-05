impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut prefix = nums[0];

        for i in 0..nums.len() {
            prefix = prefix.max(nums[i]);

            if prefix - nums[i..].iter().min().unwrap() <= k {
                return i as i32;
            }
        }

        -1
    }
}
