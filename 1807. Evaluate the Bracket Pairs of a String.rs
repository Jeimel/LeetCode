use std::collections::HashMap;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let s = s.as_bytes();
        let knowledge: HashMap<&[u8], &str> = knowledge
            .iter()
            .map(|pair| (pair[0].as_bytes(), pair[1].as_str()))
            .collect();

        let (mut i, mut start, mut result) = (0, None, String::new());

        while i < s.len() {
            match s[i] {
                b'(' if start.is_none() => {
                    i += 1;
                    start = Some(i);
                }
                b')' => {
                    result.push_str(knowledge.get(&s[start.unwrap()..i]).copied().unwrap_or("?"));
                    start = None;
                }
                _ if start.is_none() => result.push(char::from(s[i])),
                _ => {}
            };

            i += 1;
        }

        result
    }
}
