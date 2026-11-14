impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut min = i32::MAX;

        for len in (l as usize)..=(r as usize) {
            let mut sum = nums[..len].iter().sum();

            if sum > 0 {
                min = min.min(sum);
            }

            for i in len..nums.len() {
                sum += nums[i] - nums[i - len];

                if sum > 0 {
                    min = min.min(sum);
                }
            }
        }

        if min == i32::MAX { -1 } else { min }
    }
}
