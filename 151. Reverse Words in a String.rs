impl Solution {
    pub fn reverse_words(s: String) -> String {
        // Get each word by splitting the string
        s.split_ascii_whitespace()
            // Reverse the order of words
            .rev()
            .collect::<Vec<_>>()
            // Transform the words into a string again, with separating whitespaces
            .join(" ")
    }
}
