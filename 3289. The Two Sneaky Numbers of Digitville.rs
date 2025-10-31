impl Solution {
    pub fn get_sneaky_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut i, mut result) = (0, Vec::with_capacity(2));

        // We use `nums` as our hash set
        while result.len() < 2 {
            // Is the number already in the right place?
            if nums[i] == i as i32 {
                i += 1;
    
                continue;
            }

            let target = nums[i] as usize;

            // Check if the true position is already occupied,
            // if not put `nums[i]` in the correct position
            if nums[target] != nums[i] {
                nums.swap(i, target);

                continue;
            }

            // `nums[i]` already exists, so we add it to our answer
            result.push(nums[i]);
            // We remove the already existing value, so we don't count it twice
            nums.swap_remove(target);
        }

        result
    }
}
