impl Solution {
    pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
        let (s, mut adjacency) = (s.as_bytes(), vec![vec![]; parent.len()]);

        for i in 1..s.len() {
            adjacency[parent[i] as usize].push(i);
        }

        let (mut set, mut stack, mut size) = (vec![vec![]; 26], vec![(0, false)], vec![1; s.len()]);

        while let Some((node, exit)) = stack.pop() {
            let c = (s[node] - b'a') as usize;

            if exit {
                set[c].pop().unwrap();

                if node != 0 {
                    let ancestor = set[c].last().copied().unwrap_or(parent[node] as usize);
                    size[ancestor] += size[node];
                }

                continue;
            }

            set[c].push(node);
            stack.push((node, true));

            for &child in &adjacency[node] {
                stack.push((child, false));
            }
        }

        size
    }
}
