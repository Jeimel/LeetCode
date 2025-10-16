impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut congruence = vec![0; value as usize];

        for num in nums {
            // We normalize the modulo result to obtain 
            // the amount of value in the range [0, value - 1]
            let rest = ((num % value) + value) % value;

            congruence[rest as usize] += 1;
        }

        // We obtained the frequence of values in the congruence class of `value`.
        // Theoretically, we can construct all natural numbers with an infinite amount 
        // of each element due to the add operation.
        for i in 0.. {
            let rest = (i % value) as usize;

            // We check if there is still an element left, where we can
            // add `value` with in order to construct `i`
            if congruence[rest] == 0 {
                // We have found the minimum excluded, 
                // as we can't construct `i` anymore
                return i;
            }

            // We remove the used value
            congruence[rest] -= 1;
        }

        0
    }
}
