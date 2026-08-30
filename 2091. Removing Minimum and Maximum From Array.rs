impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let (mut min_num, mut min_index) = (i32::MAX, usize::MAX);
        let (mut max_num, mut max_index) = (i32::MIN, usize::MAX);

        for i in 0..nums.len() {
            if min_num > nums[i] {
                (min_num, min_index) = (nums[i], i);
            }

            if max_num < nums[i] {
                (max_num, max_index) = (nums[i], i);
            }
        }

        let (a, b) = if min_index < max_index {
            (min_index, max_index)
        } else {
            (max_index, min_index)
        };

        (a.max(b) + 1)
            .min(a + nums.len() - b + 1)
            .min(nums.len() - a.min(b)) as i32
    }
}
