impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let (mut count, mut insertions) = (0, 0);

        for b in s.bytes() {
            match b {
                b'(' if count % 2 == 1 => { insertions += 1; count += 1; },
                b'(' => count += 2,
                b')' if count < 1 => { insertions += 1; count = 1; },
                b')' => count -= 1,
                _ => {}
            }
        }

        insertions + count
    }
}
