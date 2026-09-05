impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let mut suffix = vec![nums[nums.len() - 1]; nums.len()];

        for i in (0..(nums.len() - 1)).rev() {
            suffix[i] = suffix[i + 1].min(nums[i]);
        }

        let mut prefix = nums[0];

        for i in 0..nums.len() {
            prefix = prefix.max(nums[i]);

            if prefix - suffix[i] <= k {
                return i as i32;
            }
        }

        -1
    }
}
