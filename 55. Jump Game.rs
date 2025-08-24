impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut furthest = nums[0] as usize;

        for i in 1..nums.len() {
            // Can't reach i, so there exists no solution
            if i > furthest {
                return false;
            }

            // If we can reach i, then we can calculate if the 
            // current jump length is the furthest jump so far
            furthest = furthest.max(i + nums[i] as usize);

            // If we reach or exceed the last index, we found a solution
            if furthest >= nums.len() - 1 {
                return true;
            }
        }

        // If len(nums) = 1, then we always reach the last index
        true
    }
}
