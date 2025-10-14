impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let (k, mut increasing) = (k as usize, 0);

        for i in 0..nums.len() - k - 1 {
            // We only have to check for k - 1, as this means we have found
            // an subarray fo length k due to the check to the right
            if increasing == k - 1 {
                return true;
            }

            // We parallel check if both i and i + k are strictly 
            // increasing with regard to their right neigbor
            increasing = if nums[i] < nums[i + 1] && nums[i + k] < nums[i + k + 1] {
                increasing + 1
            } else {
                0
            };
        } 

        // We have to handle the edge case for len(nums) = 2 and k = 1
        increasing == k - 1
    }
}
