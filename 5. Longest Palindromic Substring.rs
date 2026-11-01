impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let (mut bytes, mut left, mut right) = (s.as_bytes(), 0, 0);

        let expand = |mut i: usize, mut j: usize, left: &mut usize, right: &mut usize| {
            while i >= 0 && j < bytes.len() && bytes[i] == bytes[j] {
                if j - i > *right - *left {
                    (*left, *right) = (i, j);
                }

                if i < 1 {
                    break;
                }

                (i, j) = (i - 1, j + 1);
            }
        };

        for i in 0..s.len() {
            expand(i, i, &mut left, &mut right);
            expand(i, i + 1, &mut left, &mut right);
        }

        s[left..=right].to_string()
    }
}
