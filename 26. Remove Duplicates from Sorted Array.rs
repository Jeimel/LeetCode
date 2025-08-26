impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // We traverse nums using i and only increase start once per element
        let (mut i, mut start) = (0, 0);

        while i < nums.len() {
            // Remove all occurences of the current element
            while i < nums.len() -1 && nums[i] == nums[i + 1] {
                i += 1;
            }

            // Only push one occurence to the result
            nums[start] = nums[i];

            start += 1;
            i += 1;
        }

        start as i32
    }
}
