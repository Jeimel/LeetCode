use itertools::Itertools;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let Some(r) = expression.find('}') else {
            return vec![expression];
        };
        let l = expression[..r].rfind('{').unwrap();

        expression[(l + 1)..r]
            .split(',')
            .flat_map(|a| {
                Self::brace_expansion_ii([&expression[..l], a, &expression[(r + 1)..]].concat())
            })
            .sorted()
            .dedup()
            .collect()
    }
}
