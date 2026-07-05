impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let (mut right, mut count, mut total, s) = (0, [-1; 3], 0, s.as_bytes());

        for i in 0..s.len() {
            count[(s[i] - b'a') as usize] = i as i32;

            if count[0] >= 0 && count[1] >= 0 && count[2] >= 0 {
                total += 1 + count[0].min(count[1]).min(count[2]);
            }
        } 

        total
    }
}
