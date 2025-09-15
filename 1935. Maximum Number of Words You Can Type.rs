use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        // Store all broken letters in set for constant time lookup
        let broken = HashSet::<char>::from_iter(broken_letters.chars());

        // Filter out every word, which contains at least one broken letter,
        // and return the count
        text.split_ascii_whitespace()
            .filter(|word| word.chars().all(|c| !broken.contains(&c)))
            .count() as i32
   }
}
