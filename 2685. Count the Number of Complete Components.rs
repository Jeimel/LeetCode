use std::collections::VecDeque;

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut adjacent = vec![Vec::new(); n as usize];

        for edge in edges {
            let (a, b) = (edge[0] as usize, edge[1] as usize);
            adjacent[a].push(b);
            adjacent[b].push(a);
        }

        let mut visited = vec![false; adjacent.len()];
        let mut queue = VecDeque::new();
        let mut result = 0;

        let (mut edges, mut nodes) = (0, 0);

        for i in 0..visited.len() {
            if visited[i] {
                continue;
            }

            queue.push_back(i);

            edges = 0;
            nodes = 0;

            while let Some(a) = queue.pop_front() {
                if visited[a] {
                    continue;
                }
                visited[a] = true;

                nodes += 1;
                edges += adjacent[a].len();

                for &b in &adjacent[a] {
                    if !visited[b] {
                        queue.push_back(b);
                    }
                }
            }

            result += i32::from(nodes * (nodes - 1) == edges);
        }

        result
    }
}
