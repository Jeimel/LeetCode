impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let (mut x, mut result) = (0, Vec::new());
        
        for num in nums {
            // We multiply x by two, as we add a new least
            // significant bit, and we avoid overflow
            x = ((x << 1) + num) % 5;

            result.push(x == 0);
        }

        result
    }
}
