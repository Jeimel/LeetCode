impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let (mut result, mut depth) = (vec![0; seq.len()], 0);

        for (i, b) in seq.bytes().enumerate() {
            (depth, result[i]) = if b == b'(' {
                (depth + 1, (depth + 1) % 2)
            } else {
                (depth - 1, depth % 2)
            };
        }

        result
    }
}
