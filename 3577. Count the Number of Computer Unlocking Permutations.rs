impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // We can only unlock all if the complexity of the root 
        // is smaller than all other complexities
        if !complexity.iter().skip(1).all(|&c| complexity[0] < c) {
            return 0;
        }

        // Now, the number of permutations is just (n - 1)!
        (1..(complexity.len() as i64)).fold(1, |acc, c| (acc * c) % MOD) as i32
    }
}
