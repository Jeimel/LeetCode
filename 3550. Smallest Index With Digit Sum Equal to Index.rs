impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            if i > 27 {
                return -1;
            }

            let (mut num, mut sum) = (nums[i], 0);

            while num != 0 {
                sum += num % 10;
                num /= 10;
            }

            if i as i32 == sum {
                return i as i32;
            }
        }

        -1 
    }
}
