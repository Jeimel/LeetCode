use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let mut level = HashSet::from([s.clone()]);

        while !level.is_empty() {
            let result: Vec<String> = level.iter().filter_map(|item| Self::is_valid(item).then(|| item.clone())).collect();

            if !result.is_empty() {
                return result;
            }

            let mut next = HashSet::new();

            for item in &level {
                for i in 0..item.len() {
                    let mut s = String::with_capacity(item.len() - 1);
                    s.push_str(&item[0..i]);
                    s.push_str(&item[(i + 1)..]);

                    next.insert(s); 
                }
            }

            level = next;
        }

        unreachable!()
    }

    fn is_valid(s: &String) -> bool {
        let mut count = 0;

        for b in s.bytes() {
            match b {
                b'(' => count += 1,
                b')' => count -= 1,
                _ => {}
            }

            if count < 0 {
                return false;
            }
        }

        count == 0
    }
}
