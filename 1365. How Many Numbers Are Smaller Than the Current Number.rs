impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted = nums.clone();
        sorted.sort();

        nums
            .iter()
            .map(|&i| sorted.partition_point(|&j| j < i) as i32)
            .collect()
    }
}
