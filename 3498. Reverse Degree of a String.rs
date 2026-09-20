impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut degree = 0;

        for (i, c) in s.bytes().enumerate() {
            degree += (26 - (c - b'a')) as i32 * (i as i32 + 1);
        }

        degree
    }
}
