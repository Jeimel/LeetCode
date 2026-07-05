use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect();

        while let Some(y) = heap.pop() {
            let Some(x) = heap.pop() else {
                return y;
            };

            if y != x {
                heap.push(y - x);
            }
        }

        0 
    }
}
