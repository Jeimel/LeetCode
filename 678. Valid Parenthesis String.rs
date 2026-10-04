impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let (mut low, mut high) = (0, 0);

        for b in s.bytes() {
            match b {
                b'(' => { low += 1; high += 1; },
                b')' => { low -= 1; high -= 1; },
                b'*' => { low -= 1; high += 1; },
                _ => {},
            }

            low = low.max(0);

            if high < 0 {
                return false;
            }
        }

        low == 0
    }
}
