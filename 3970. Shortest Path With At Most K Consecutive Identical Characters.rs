use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let (labels, n, k) = (labels.as_bytes(), n as usize, k as usize);

        let mut adjacency = vec![vec![]; n];

        for edge in edges {
            adjacency[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        let mut dist = vec![vec![i32::MAX; k]; n];
        let mut heap = BinaryHeap::new();

        dist[0][0] = 0;
        heap.push(Reverse((0, 0, 0)));

        while let Some(Reverse((cost, count, node))) = heap.pop() {
            if node == n - 1 {
                return cost;
            }

            if cost > dist[node][count] {
                continue;
            }

            let label = labels[node];

            for edge in &adjacency[node] {
                let next = (
                    cost + edge.1,
                    (count + 1) * usize::from(label == labels[edge.0]),
                    edge.0,
                );

                if next.1 < k && next.0 < dist[next.2][next.1] {
                    heap.push(Reverse(next));
                    dist[next.2][next.1] = next.0;
                }
            }
        }

        -1
    }
}
