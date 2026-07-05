impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = 0;

        for i in 0..nums.len() {
            let mut count = 0;

            for &num in &nums[i..] {
                count += -1 + i32::from(num == target) * 2;
                result += i32::from(count > 0);
            }
        }

        result
    }
}
