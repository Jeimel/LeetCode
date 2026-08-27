impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut set = [false; 100];

        for num in nums {
            set[num as usize - 1] = true;
        }

        let mut i = k;

        while i <= set.len() {
            if !set[i - 1] {
                return i as i32;
            }

            i += k;
        }

        i as i32
    }
}
