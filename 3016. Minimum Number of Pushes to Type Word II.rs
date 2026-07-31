use std::cmp::Reverse;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut buckets = vec![0; 26];

        for letter in word.bytes() {
            buckets[(letter - b'a') as usize] += 1;
        }

        buckets.sort_unstable_by_key(|&count| Reverse(count));

        let mut count = 0;

        for i in 0..buckets.len() {
            if buckets[i] == 0 {
                continue;
            }

            count += ((i / 8) + 1) * buckets[i];
        }

        count as i32
    }
}
