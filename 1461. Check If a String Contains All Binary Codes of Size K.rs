use std::collections::HashSet;

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        let k = k as usize;
        let set: HashSet<&str> = HashSet::from_iter((k..(s.len() + 1)).map(|i| &s[(i - k)..i]));

        set.len() == 1 << k
    }
}
