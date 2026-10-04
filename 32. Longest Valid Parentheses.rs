impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();

        let mut max = 0;
        let (mut forward_len, mut forward_count) = (0, 0);
        let (mut backward_len, mut backward_count) = (0, 0);

        for i in 0..s.len() {
            match s[i] {
                b'(' => forward_count += 1,
                b')' => forward_count -= 1,
                _ => {}
            }

            match s[s.len() - i - 1] {
                b'(' => backward_count -= 1,
                b')' => backward_count += 1,
                _ => {}
            }

            forward_len += 1;
            backward_len += 1;

            if forward_count == 0 {
                max = max.max(forward_len);
            }

            if backward_count == 0 {
                max = max.max(backward_len);
            }

            if forward_count < 0 {
                (forward_count, forward_len) = (0, 0);
            }

            if backward_count < 0 {
                (backward_count, backward_len) = (0, 0);
            }
        }

        max
    }
}
