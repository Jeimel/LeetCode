impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let (mut selections, mut left, mut right) = (0, 0, nums.iter().sum::<i32>());

        for num in nums {
            if num != 0 {
                left += num;
                right -= num;

                continue;
            }

            // We can just track the sum of all elements to the left and right.
            // If we find an zero-element, we compute the difference of left - right,
            // respectively right - left, depending on the direction. Because we bounce 
            // alternately on both sides, the difference for a valid solution is either
            // 0 or 1.

            // Do we exceed the limit when initially moving to the left?
            if left - right >= 0 && left - right <= 1 {
                selections += 1;
            }

            // Do we exceed the limit when initially moving to the right?
            if right - left >= 0 && right - left <= 1 {
                selections += 1;
            }
        }

        selections
    }
}
