use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut words_set = HashSet::with_capacity(wordlist.len());
        let mut words_insensitive = HashMap::with_capacity(wordlist.len());
        let mut words_vowel = HashMap::with_capacity(wordlist.len());

        // We only insert the first match into insensitive and vowel
        for word in &wordlist {
            words_set.insert(word);
            words_insensitive.entry(word.to_lowercase()).or_insert(word);
            words_vowel.entry(Self::remove_vowels(word)).or_insert(word);
        }

        queries
            .iter()
            .map(|query| {
                Self::spellcheck(
                    &query.to_string(),
                    &wordlist,
                    &words_set,
                    &words_insensitive,
                    &words_vowel,
                )
            })
            .collect()
    }

    fn spellcheck(
        query: &String,
        words: &Vec<String>,
        words_set: &HashSet<&String>,
        words_insensitive: &HashMap<String, &String>,
        words_vowel: &HashMap<String, &String>,
    ) -> String {
        //We have to check for every query, if present in the wordlist
        if words_set.contains(query) {
            return query.clone();
        }

        // We check for insensitive matching next by converting the query to lowercase
        let query_insensitive = query.to_lowercase();

        // We can use the HashMap lookup, as we only inserted the first match
        if let Some(word) = words_insensitive.get(&query_insensitive) {
            return word.to_string();
        }

        // We remove all vowels, and replace them with '*' to find words with equal consonants
        let query_vowel = Self::remove_vowels(&query);

        // Again, we can use the HashMap to search for the first match
        if let Some(word) = words_vowel.get(&query_vowel) {
            return word.to_string();
        }

        // We found no matches, so we return an empty string
        String::new()
    }

    fn remove_vowels(word: &str) -> String {
        word.to_lowercase()
            .chars()
            .map(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '*',
                _ => c,
            })
            .collect()
    }
}
