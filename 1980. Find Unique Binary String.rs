impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = 1 << nums[0].len();
        let mut set = vec![false; n];

        for num in &nums { 
            set[usize::from_str_radix(num, 2).unwrap()] = true;
        }

        for i in 0..set.len() {
            if !set[i] {
                return format!("{:0n$b}", i, n = nums[0].len());
            }
        }

        unreachable!()
    }
}
