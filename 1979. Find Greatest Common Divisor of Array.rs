impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let (mut a, mut b) = nums.iter().fold((i32::MAX, i32::MIN), |(min, max), &num| {
            (min.min(num), max.max(num))
        });

        while b != 0 {
            (a, b) = (b, a % b);
        }

        a
    }
}
