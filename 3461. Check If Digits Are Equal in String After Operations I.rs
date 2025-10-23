impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits: Vec<u8> = s.bytes().map(|b| b - b'0').collect();

        // We just apply the operation and update our result until we reach the desired length
        while digits.len() > 2 {
            let mut next = Vec::new();

            for i in 0..digits.len() - 1 {
                // Due to overflow we have to take the modulo in each iteration
                next.push((digits[i] + digits[i + 1]) % 10);
            }

            digits = next;
        }
        
        digits[0] == digits[1]
    }
}
