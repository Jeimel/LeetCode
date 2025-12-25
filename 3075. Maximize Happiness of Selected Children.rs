use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let (mut heap, mut sum) = (BinaryHeap::from(happiness), 0);

        // We use a heap to only sort the first `k` values
        for i in 0..(k as i64) {
            sum += (heap.pop().unwrap() as i64 - i).max(0);
        }

        sum
    }
}
