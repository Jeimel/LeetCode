impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack = vec![vec![]];

        for b in s.bytes() {
            match b {
                b'(' => stack.push(Vec::new()),
                b')' => {
                    let mut top = stack.pop().unwrap();
                    top.reverse();

                    (*stack.last_mut().unwrap()).extend_from_slice(&top);
                },
                _ => (*stack.last_mut().unwrap()).push(b),
            };
        }

        String::from_utf8(stack.last().unwrap().to_vec()).unwrap()
    }
}
