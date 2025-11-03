impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        // We directly transform the inner bytes of `s`
        let mut bytes = unsafe { s.as_bytes_mut() };

        let (mut left, mut right) = (0, bytes.len() - 1);

        // We increase/decrease `left` and `right` until we both index a vowel,
        // which are then swapped and both pointer are updated
        while left < right {
            if !matches!(bytes[left].to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u') {
                left += 1;
                continue;
            }

            if !matches!(bytes[right].to_ascii_lowercase(), b'a' | b'e' | b'i' | b'o' | b'u') {
                right -= 1;
                continue;
            }

            bytes.swap(left, right);

            left += 1;
            right -= 1;
        }

        s
    }
}
