impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        // Just brute-force all numbers greater than n
        for i in (n as usize + 1).. {
            if let Some(count) = &Self::count(i) && Self::verify(count) {
                return i as i32;
            }
       }

        unreachable!()
    }

    fn count(n: usize) -> Option<[usize; 10]> {
        let (mut count, mut n) = ([0; 10], n);

        while n != 0 {
            let digit = n % 10;
            count[digit] += 1;

            // We can skip the count if we encounter a digit greater than 6 or 0,
            // and if the count already exceeds the digit
            if digit > 6 || digit == 0 || count[digit] > digit {
                return None;
            }

            n /= 10;
        }

        Some(count)
    }

    fn verify(count: &[usize; 10]) -> bool {
        count.iter().enumerate().all(|(i, count)| *count == 0 || i == *count)
    }
}
