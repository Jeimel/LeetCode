impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let (mut previous, k) = (s, k as usize);

        while previous.len() > k {
            let mut current = String::new();

            for chunk in previous.as_bytes().chunks(k) {
                let sum: i32 = chunk.iter().map(|d| (d - b'0') as i32).sum();
                current.push_str(&sum.to_string());
            }

            previous = current;
        }

        previous
    }
}
