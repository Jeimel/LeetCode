use std::collections::VecDeque;

impl Solution {
    pub fn remaining_methods(n: i32, k: i32, invocations: Vec<Vec<i32>>) -> Vec<i32> {
        let (n, k) = (n as usize, k as usize);

        let mut adjacency = vec![vec![]; n];
        
        for edge in invocations {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            adjacency[a].push(b);
        }

        let mut queue = VecDeque::new();
        let mut visited = vec![false; n];

        queue.push_back(k);

        while let Some(method) = queue.pop_front() {
            if visited[method] {
                continue;
            }

            visited[method] = true;

            for &invocation in &adjacency[method] {
                queue.push_back(invocation);
            }
        }

        let mut not_malicious = Vec::new();

        for (method, &visit) in visited.iter().enumerate() {
            if visit {
                continue;
            }

            not_malicious.push(method as i32);

            for &invocation in &adjacency[method] {
                if visited[invocation] {
                    return (0..(n as i32)).collect();
                }
            }
        }

        not_malicious
    }
}
