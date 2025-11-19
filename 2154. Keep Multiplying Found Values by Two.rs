impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let (mut exists, mut original) = ([false; 1000], original as usize);

        for num in nums {
            exists[num as usize - 1] = true;
        }

        loop {
            if original > 1000 || !exists[original - 1]{
                return original as i32;
            }

            original *= 2;
        } 
    }
}
