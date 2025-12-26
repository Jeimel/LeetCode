impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let mut suffix = customers.chars().filter(|c| *c == 'Y').count();
        let (mut best, mut max) = (suffix, 0);

        for (i, c) in customers.chars().enumerate() {
            suffix = if c == 'Y' { suffix - 1 } else { suffix + 1 };

            if suffix < best {
                (best, max) = (suffix, i + 1);
            }
        }

        max as i32
    }
}
