impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let (mut current, mut last, mut max) = (1, 0, 0);

        for i in 1..nums.len() {
            // We increase our counter if the subarray is still increasing,
            // or store the counter and reset it afterwards
            current = if nums[i - 1] >= nums[i] {
                last = current;
                1
            } else {
                current + 1
            };

            // We need to check for two cases:
            // 1: Can we combine the current and previous subarrays?
            // 2: Can we divide our current subarray in half to find a new maximum?
            max = max.max(current.min(last));
            max = max.max(current / 2);
        }

        max
    }
}
