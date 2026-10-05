impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let (mut prev, mut score, mut depth) = (b')', 0, 1);

        for b in s.bytes() {
            match b {
                b'(' => depth *= 2,
                b')' if prev == b'(' => { depth /= 2; score += depth; }
                b')' => depth /= 2,
                _ => {},
            };

            prev = b;
        }

        score
   }
}
