impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let (mut i, mut result) = (0, vec![]);

        while i < nums.len() {
            let j = nums[i] as usize - 1;

            if nums[i] != nums[j] {
                nums.swap(i, j);
            } else {
                i += 1;
            }
        }

        for i in 0..nums.len() {
            let j = i as i32 + 1;

            if j != nums[i] {
                result.push(j);
            }
        }

        result
    }
}
