impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let (mut frequency, s, mut left, mut max) = ([0; 26], s.as_bytes(), 0, 0);

        for right in 0..s.len() {
            let a = (s[right] - b'a') as usize;

            frequency[a] += 1;

            while frequency[a] > 2 {
                frequency[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }

            max = max.max(right - left + 1);
        }

        max as i32
    }
}
