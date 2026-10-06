impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let (mut count, mut insertions) = (0i32, 0i32);

        for b in s.bytes() {
            (count, insertions) = if b == b'(' {
                (count + 1, insertions)
            } else if count > 0 {
                (count - 1, insertions)
            } else {
                (count, insertions + 1)
            }
        }

        insertions + count
    }
}
