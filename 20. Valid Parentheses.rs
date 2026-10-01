impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for b in s.bytes() {
            match b {
                b'(' => stack.push(b')'),
                b'{' => stack.push(b'}'),
                b'[' => stack.push(b']'),
                _ if !stack.is_empty() && b == stack.pop().unwrap() => continue,
                _ => return false,
            }
        }

        stack.is_empty()
    }
}
