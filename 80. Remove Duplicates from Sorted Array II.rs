impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // For len(nums) <= 2 we don't have to remove any element.
        // However for len(nums) <= 1 we have to return the length 
        // of the array and due to the if-statement i must start at 2
        let mut i = nums.len().min(2);

        for j in 2..nums.len() {
            // If nums[j] and nums[i - 2] are equal,
            // we already have two occurences of the given element
            if nums[j] == nums[i - 2] {
                continue;
            }

            nums[i] = nums[j];
            // i tracks the elements, which are present in the final result
            i += 1;
        }

        i as i32
    }
}
