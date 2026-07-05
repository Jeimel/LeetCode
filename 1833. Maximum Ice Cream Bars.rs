use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(costs.iter().map(|cost| Reverse(cost)));

        let (mut price, mut count) = (0, 0);

        while let Some(Reverse(&top)) = heap.pop() && price + top <= coins {
            price += top;
            count += 1;
        }

        count
    }
}
