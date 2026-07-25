impl Solution {
    pub fn max_product(mut n: i32) -> i32 {
        let (mut current, mut previous) = (0, 0);

        while n != 0 {
            let digit = n % 10;
            n /= 10;

            if digit >= current {
                (current, previous) = (digit, current);
            } else if digit >= previous {
                previous = digit;
            }
        }

        current * previous
    }
}
