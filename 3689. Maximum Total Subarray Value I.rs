impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let (min, max) = nums.iter().fold((i32::MAX, i32::MIN), |(min, max), &num| {
            (min.min(num), max.max(num))
        });

        (max - min) as i64 * k as i64
    }
}
