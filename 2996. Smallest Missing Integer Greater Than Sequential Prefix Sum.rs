impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let (mut i, mut prefix) = (1, nums[0] as usize);

        while i < nums.len() && nums[i] == nums[i - 1] + 1 {
            prefix += nums[i] as usize;
            i += 1;
        }

        let mut set = vec![false; 51];

        i -= 1;

        while i < nums.len() {
            set[nums[i] as usize] = true;
            i += 1;
        }

        while prefix < set.len() && set[prefix] {
            prefix += 1;
        }

        prefix as i32
    }
}
