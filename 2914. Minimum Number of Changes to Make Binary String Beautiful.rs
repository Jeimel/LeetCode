impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut changes = 0;

        for chunk in s.as_bytes().chunks(2) {
            changes += i32::from(chunk[0] != chunk[1]);
        }

        changes
    }
}
