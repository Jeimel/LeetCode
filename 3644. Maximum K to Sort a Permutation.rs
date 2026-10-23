impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut and = i32::MAX;

        for i in 0..nums.len() {
            if nums[i] != i as i32 {
                and &= nums[i];
            }
        }

        if and == i32::MAX { 0 } else { and }
    }
}
