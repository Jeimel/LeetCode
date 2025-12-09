impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const M: usize = 100001;
        const MOD: i64 = 1000000007;

        let (mut prev, mut next, mut count) = ([0; M], [0; M], 0);

        for &num in &nums {
            next[num as usize] += 1;
        }

        for i in 0..nums.len() - 1 {
            let num = nums[i] as usize;

            next[num] -= 1;

            let j = num * 2;
            if j < M {
                count += prev[j] as i64 * next[j] as i64;
            }

            prev[num] += 1;
        }

        (count % MOD) as i32
    }
}
