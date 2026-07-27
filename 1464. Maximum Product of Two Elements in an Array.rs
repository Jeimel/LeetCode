impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut current, mut previous) = (0, 0);

        for num in nums {
            if num > current {
                (current, previous) = (num, current);
            } else if num > previous {
                previous = num;
            }
        }

        (current - 1) * (previous - 1)
    }
}
