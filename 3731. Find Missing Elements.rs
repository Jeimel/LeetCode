impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        (1..nums.len())
            .map(|i| ((nums[i - 1] + 1)..nums[i]))
            .flatten()
            .collect()
    }
}
