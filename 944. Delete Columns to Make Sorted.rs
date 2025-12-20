impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let rows: Vec<_> = strs.iter().map(|row| row.as_bytes()).collect();
        let mut deleted = 0;

        for i in 0..rows[0].len() {
            for j in 1..rows.len() {
                if rows[j][i] >= rows[j - 1][i] {
                    continue;
                }

                deleted += 1;
                break;
            }
        }

        deleted
    }
}
