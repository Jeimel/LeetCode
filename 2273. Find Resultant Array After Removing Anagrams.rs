impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        // We store the recently tested word, and it's frequency
        let (mut stack, mut result) = (vec![Self::frequency(&words[0])], vec![words[0].clone()]);

        for i in 1..words.len() {
            // Get the frequency of the next word
            let frequency = Self::frequency(&words[i]);

            // and check the recent word is an anagram
            if frequency == *stack.last().unwrap() {
                continue;
            }

            // We found a new word, so we have to include it into the solution,
            // as we are only considering anagrams to the right of our new word
            stack.push(frequency);
            result.push(words[i].clone());
        }

        result
    }

    // The input only consist of lowercase english letters,
    // so we can restrict the frequency to an array of length 26
    fn frequency(word: &str) -> [u8; 26] {
        word.chars().fold([0; 26], |mut acc, c| {
            acc[c as usize - 'a' as usize] += 1;
            acc
        })
    }
}
