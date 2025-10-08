impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        const MAX: usize = 10_000_000_000 / 100_000;

        let (success, mut suffixes) = (success as usize, [0; MAX + 1]);

        // Count frequency of each potion
        for potion in potions {
            suffixes[potion as usize] += 1;
        }

        // Aggregate frequencies from right to left, which allows to get the amount 
        // of values bigger or equal than our minimum potion value in constant time
        suffixes.iter_mut().rev().fold(0, |sum, suffix| {
            *suffix += sum;
            *suffix
        });

        // Now, we just have to calculate our minimum potion value, and update the vector.
        // Potion values greater than `MAX` are not possible, so the count is zero
        for spell in &mut spells {
            *spell = *suffixes.get(success.div_ceil(*spell as usize)).unwrap_or(&0);
        }

        spells
   }
}
