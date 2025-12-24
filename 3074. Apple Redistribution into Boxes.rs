impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        capacity.sort_unstable_by(|a, b| b.cmp(a));

        let (mut sum, mut boxes) = (apple.iter().sum::<i32>(), 0);

        while sum > 0 {
            sum -= capacity[boxes];
            boxes += 1;
        }

        boxes as i32
    }
}
