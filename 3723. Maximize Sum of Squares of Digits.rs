impl Solution {
    pub fn max_sum_of_squares(mut num: i32, mut sum: i32) -> String {
        if sum > 9 * num {
            return String::new();
        }

        let nines = sum as usize / 9;
        let remaining = sum % 9;

        let mut integer = "9".repeat(nines);

        if remaining > 0 {
            integer.push(char::from(remaining as u8 + b'0'))
        }

        integer.push_str(&"0".repeat(num as usize - integer.len()));

        integer
    }
}
