impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        // Calculate the amount of steps for each query and sum the results
        queries
            .iter()
            .map(|query| Self::make_zero(query[0] as i64, query[1] as i64))
            .sum()
    }

    fn make_zero(l: i64, r: i64) -> i64 {
        // We have to store the current lower bound, the number of operations for
        // each number to reach zero, and the current exponent, which is equal
        // to the highest power of two smaller than l
        let (mut i, mut exponent, mut steps) = (l, l.ilog(4) as i64 + 1, 0);

        // We always calculate the amount of numbers, which are in the
        // range of [i, 4^exponent). As these require the same amount
        // of steps to reach zero, so we can group these
        loop {
            // Calculate the next power of two as upper bound for the current group
            let power = 1 << 2 * exponent;

            // Did you exceed the interval?
            if power > r {
                // r is smaller than power, so there are only r - i + 1 numbers left.
                // We have to add one, as the query is inclusive
                steps += exponent * (r - i + 1);

                break;
            }

            // We need exponent amount of steps to reach 0 for each element in
            // the current group, and there are power - i elements, as i represents
            // the current lower bound (either l or the last power of four) and power
            // is the next power of four (the edge case of power > r is already handled).
            // Therefore, we need exponent * (power - i) steps to reach zero in this group
            steps += exponent * (power - i);
            // Increment the exponent, as we check the next power of four in the
            // following itereation
            exponent += 1;
            // power is the next lower bound
            i = power;
        }

        // We can always select two integers to apply our operation to, so we can divide
        // the amount of steps by two. However, we have to ceil the result, because we 
        // have to apply the operation again if there is an integer left
        (steps + 1) / 2
    }
}
