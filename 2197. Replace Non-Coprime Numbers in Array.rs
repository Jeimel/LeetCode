impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut stack = Vec::new();

        for x in nums {
            let mut lcm = x as i64;

            // We try to combine x with all previous elements until
            // we found a coprime number for the current element
            while let Some(&y) = stack.last() {
                let gcd = Self::gcd(lcm, y);

                // Are x and y coprime? If yes, we have to keep x and y in the stack
                if gcd <= 1 {
                    break;
                }

                lcm = Self::lcm(lcm, y, gcd);
                // Remove y from the stack, as already combined it with the current element
                stack.pop();
            }

            // We either push the original x, or the combined lcm
            stack.push(lcm);
        }

        stack.into_iter().map(|x| x as i32).collect()
    }

    // lcm(x, y) = x * y / gcd(x, y)
    fn lcm(x: i64, y: i64, gcd: i64) -> i64 {
        x * y / gcd
    }

    fn gcd(x: i64, y: i64) -> i64 {
        // Base case: gcd(x, 0) = x
        if y == 0 { 
            return x; 
        }
        
        // We know that gcd(x, y) = gcd(y, x % y), because if a number divides both x and y, 
        // it must also divide the remainder x % y.
        Self::gcd(y, x % y)
    }
}
