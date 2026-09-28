impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.bytes().fold((i32::MIN, 0), |(max, mut current), b| {
            match b {
                b'(' => current += 1,
                b')' => current -= 1,
                _ => {},
            };

            (max.max(current), current)
        }).0
    }
}
