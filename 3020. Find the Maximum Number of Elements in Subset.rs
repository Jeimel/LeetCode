use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let mut count = nums.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        });

        let mut max = if let Some(count) = count.remove(&1) {
            count - i32::from(count % 2 == 0)
        } else {
            0
        };

        for &num in count.keys() {
            let (mut result, mut x) = (0, num);

            while let Some(&count) = count.get(&x) && count > 1 {
                result += 2;
                x *= x;
            }

            max = max.max(result + if count.contains_key(&x) { 1 } else { - 1 });
        }

        max
    }
}
