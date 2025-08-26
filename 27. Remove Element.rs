impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len());

        while left < right {
            // We just have to increase the left pointer if nums[left]
            // does not equal the target
            if nums[left] != val {
                left += 1;

                continue;
            }

            // If we find the target, we move the element to the back of the array
            right -= 1;
            // We don't increase left, as nums[right] could also equal the target
            nums.swap(left, right);
        }

        // left equals the number of elements, which do not equal the target
        left as i32
    }
}
