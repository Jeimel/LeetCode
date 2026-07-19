impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let s: Vec<(usize, usize)> = s.bytes().map(|b| (b - b'a') as usize).enumerate().collect();

        let mut stack = Vec::new();
        let mut visited = [false; 26];
        let mut last = s.iter().fold([0; 26], |mut last, &(i, a)| {
            last[a] = i;
            last
        });

        for &(i, a) in &s {
            if visited[a] {
                continue;
            }

            while let Some(&b) = stack.last() && a < b && last[b] > i {
                visited[b] = false;
                stack.pop();
            }

            visited[a] = true;
            stack.push(a);
        }

        stack.iter().map(|&a| (a as u8 + b'a') as char).collect()
    }
}
