impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        // Due to the constraints of n a linear search is feasible.
        // We have two pointer, one starting at 1 and one starting at n - 1.
        for a in 1..n {
            // The sum of a and b will always equal n
            let b = n - a;

            // Have we found two no-zero integers, which are equal to n?
            if !(Self::no_zero(a) && Self::no_zero(b)) {
                continue;
            }

            return vec![a, b];
        }

        // There is at least one valid solution 
        unreachable!()
    }

    fn no_zero(mut n: i32) -> bool {
        // We iterate over every digit and check if it is divisible by ten. 
        // In other words, we check if each digiti is not equal to zero.
        while n > 0 {
            // We only consider the first digit and check if equal to zero
            if n % 10 == 0 {
                return false;
            }

            // Strip the first digit
            n /= 10;
        }

        true
    }
}
