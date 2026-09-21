impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let mut dp = vec![vec![0; k as usize]; nums.len()];
        let mut result = vec![0; dp[0].len()];

        let k = k as usize;
        let x = nums[0] as usize % k;

        dp[0][x] = 1;
        result[x] = 1;

        for i in 1..dp.len() {
            let x = nums[i] as usize % k;

            for j in 0..dp[i].len() {
                dp[i][(j * x) % k] += dp[i - 1][j];
            }

            dp[i][x] += 1; 
        
            for j in 0..dp[i].len() {
                result[j] += dp[i][j]
            }
        }

        result
    }
}
