use std::collections::VecDeque;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize; 

        let mut in_degree = vec![0; num_courses];
        let mut adjacency = vec![Vec::new(); num_courses];

        for edge in prerequisites {
            let (a, b) = (edge[0] as usize, edge[1] as usize);

            adjacency[b].push(a);
            in_degree[a] += 1;
        }

        let mut queue: VecDeque<usize> = (0..num_courses).filter(|i| in_degree[*i] == 0).collect();
        let mut order = Vec::new();

        while let Some(a) = queue.pop_front() {
            order.push(a as i32);

            for &b in &adjacency[a] {
                in_degree[b] -= 1;
                if in_degree[b] == 0 {
                    queue.push_back(b);
                }
            }
        }

        if order.len() == num_courses { order } else { vec![] }
    }
}
