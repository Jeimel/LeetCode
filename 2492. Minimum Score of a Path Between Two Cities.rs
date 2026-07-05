use std::collections::VecDeque;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut adjacency = vec![Vec::new(); n];
        
        for road in roads {
            let (a, b, distance) = (road[0] as usize, road[1] as usize, road[2]);
            adjacency[a - 1].push((b - 1, distance));
            adjacency[b - 1].push((a - 1, distance));
        }

        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        let mut min_distance = i32::MAX;

        queue.push_front(0);

        while let Some(a) = queue.pop_front() {
            visited[a] = true;

            for &(b, distance) in &adjacency[a] {
                if visited[b] {
                    continue;
                }

                min_distance = min_distance.min(distance);
                queue.push_back(b);
            }
        }

        min_distance
    }
}
