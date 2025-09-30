impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        // We have to repeat the process until n == 1
        while nums.len() > 1 {
            // Apply the operation for the first n - 1 numbers
            for i in 0..nums.len() - 1 {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }

            // Remove the last number in-place to prevent extra memory
            nums.pop();
        }

        // We only have one number left
        nums[0]
    }
}
