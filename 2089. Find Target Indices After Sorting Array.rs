impl Solution {
    pub fn target_indices(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        nums.sort_unstable();

        let i = nums.partition_point(|&num| num < target) as i32;
        let j = nums.partition_point(|&num| num <= target) as i32;

        (i..j).collect()
    }
}
