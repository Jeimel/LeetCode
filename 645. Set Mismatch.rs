impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut set = vec![false; nums.len() + 1];
        let mut twice = -1;

        for num in nums {
            let i = num as usize;
            
            if set[i] {
                twice = num;
            }

            set[i] = true;
        }

        vec![twice, set.iter().skip(1).position(|s| !s).unwrap() as i32 + 1]
    }
}
