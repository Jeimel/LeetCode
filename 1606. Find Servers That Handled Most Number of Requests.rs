use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap};

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        let k = k as usize;

        let mut busy = BinaryHeap::new();
        let mut available = BTreeSet::from_iter((0..k));
        let mut requests = vec![0; k];

        for i in 0..arrival.len() {
            while let Some(&Reverse((time, j))) = busy.peek()
                && time <= arrival[i]
            {
                busy.pop().unwrap();
                available.insert(j);
            }

            let Some(j) = available
                .range((i % k)..)
                .next()
                .copied()
                .or_else(|| available.iter().next().copied())
            else {
                continue;
            };

            busy.push(Reverse((arrival[i] + load[i], j)));
            available.remove(&j);
            requests[j] += 1;
        }

        let max = *requests.iter().max().unwrap();

        requests
            .iter()
            .enumerate()
            .filter_map(|(i, &requests)| (requests == max).then_some(i as i32))
            .collect()
    }
}
