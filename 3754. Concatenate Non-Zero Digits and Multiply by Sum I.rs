impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let (mut n, mut sum, mut x, mut exp) = (n as i64, 0, 0, 1);

        while n != 0 {
            let digit = n % 10;
            n /= 10;

            if digit == 0 {
                continue;
            }

            sum += digit;
            x += digit * exp;
            exp *= 10;
        }

        x * sum
    }
}
