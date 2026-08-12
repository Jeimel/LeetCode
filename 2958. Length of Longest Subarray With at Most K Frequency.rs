use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let (mut count, mut left, mut max) = (HashMap::<i32, i32>::new(), 0, 0);

        for right in 0..nums.len() {
            *count.entry(nums[right]).or_insert(0) += 1;

            while *count.get(&nums[right]).unwrap() > k {
                *count.get_mut(&nums[left]).unwrap() -= 1;
                left += 1;
            }

            max = max.max(right - left + 1);
        }

        max as i32
    }
}
