impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // The majority element must appear more than n / 2 times.
        // Therefore, count must be positive for the correct candidate,
        // as the majority element can not be canceled out
        let (mut count, mut candidate) = (1, nums[0]);

        for i in 1..nums.len() {
            // We don't have any potential majority element at the moment
            if count == 0 {
                candidate = nums[i];
            }

            // Count is increased for each occurence of the current candidate
            // and decreased for each other element
            count += if nums[i] == candidate { 1 } else { -1 };
        }

        candidate
    }
}
