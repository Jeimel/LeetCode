impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut square: Vec<i64> = nums.iter().map(|&num| (num as i64) * (num as i64)).collect();
        let (less, median, greater) = square.select_nth_unstable_by_key(nums.len() / 2, |&num| num);

        *median + greater.iter().sum::<i64>() - less.iter().sum::<i64>()
    }
}
