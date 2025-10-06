impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        // We create a sliding window, and track the maximum
        let (mut left, mut right, mut max) = (0, height.len() - 1, 0);

        while left < right {
            // We calculate the area based on our pointer, and update the maximum if necessary
            max = max.max(height[left].min(height[right]) * (right - left) as i32);

            // We move the pointer, which represents a lower height
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max
    }
}
