impl Solution {
    pub fn check_string(s: String) -> bool {
        s.find('b')
            .map(|i| s[i..].find('a').is_none())
            .unwrap_or(true)
    }
}
