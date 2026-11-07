use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut available: BinaryHeap<_> = servers
            .iter()
            .enumerate()
            .map(|(i, &weight)| Reverse((weight, i as i32)))
            .collect();
        let mut processing: BinaryHeap<Reverse<(i32, i32, i32)>> = BinaryHeap::new();
        let mut result = Vec::new();
        let mut time = 0;

        for i in 0..tasks.len() {
            time = (i as i32).max(if processing.len() == servers.len() {
                processing.peek().unwrap().0.0
            } else {
                time
            });

            while let Some(&Reverse((second, weight, j))) = processing.peek()
                && second <= time
            {
                processing.pop().unwrap();
                available.push(Reverse((weight, j)));
            }

            let Reverse((weight, j)) = available.pop().unwrap();

            result.push(j);
            processing.push(Reverse((time + tasks[i], weight, j)));
        }

        result
    }
}
