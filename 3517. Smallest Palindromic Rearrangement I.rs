impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut bytes: Vec<_> = s.bytes().collect();
        let mut buckets = [0; 26];

        for i in 0..(s.len() / 2) {
            buckets[(bytes[i] - b'a') as usize] += 1;
        }

        let mut count = 0;

        for i in 0..buckets.len() {
            for j in 0..buckets[i] {
                let c = i as u8 + b'a';

                bytes[count] = c;
                bytes[s.len() - 1 - count] = c;

                count += 1;
            }
        }

        String::from_utf8(bytes).unwrap()
    }
}
