impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = [0; 30];

        for num in nums {
            for i in 0..bits.len() {
                bits[i] += i32::from((num & (1 << i)) != 0);
            }
        }

        let mut result = 0;

        for _ in 0..k {
            let mut num = 0;

            for i in 0..bits.len() {
                num |= i64::from(bits[i] > 0) << i;
                bits[i] -= 1;
            }

            result = (result + num * num) % 1_000_000_007i64;
        }

        result as i32
    }
}
