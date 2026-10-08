impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let (mut count, mut result) = (0, String::new());

        for c in s.chars() {
            match c {
                '(' if count == 0 => count += 1,
                '(' => { count += 1; result.push(c); },
                ')' if count == 1 => count -= 1,
                ')' => { count -= 1; result.push(c); },
                _ => {},
            }
        }

        result
    }
}
