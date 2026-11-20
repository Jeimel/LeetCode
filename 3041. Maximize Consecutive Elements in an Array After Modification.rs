use std::collections::HashMap;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let (mut answer, mut dp) = (0, HashMap::new());

        for num in nums {
            let incremented = dp.get(&num).copied().unwrap_or(0) + 1;
            dp.insert(num + 1, incremented);

            let unchanged = dp.get(&(num - 1)).copied().unwrap_or(0) + 1;
            dp.insert(num, unchanged);

            answer = answer.max(unchanged).max(incremented);
        }

        answer
    }
}
