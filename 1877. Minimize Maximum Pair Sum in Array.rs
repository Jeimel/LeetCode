impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut max = nums[0] + nums[nums.len() - 1];

        for i in 1..(nums.len() / 2) {
            max = max.max(nums[i] + nums[nums.len() - 1 - i]);
        }

        max

    }
}
