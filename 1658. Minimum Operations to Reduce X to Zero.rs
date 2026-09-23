impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;

        if target < 0 {
            return -1;
        }

        if target == 0 {
            return nums.len() as i32;
        }

        let (mut max, mut sum, mut i) = (0, 0, 0);
        
        for j in 0..nums.len() {
            sum += nums[j];

            while sum > target {
                sum -= nums[i];
                i += 1;
            }

            if sum == target {
                max = max.max(j - i + 1)
            }
        }

        if max == 0 {
            return -1;
        }

        (nums.len() - max) as i32
    }
}
