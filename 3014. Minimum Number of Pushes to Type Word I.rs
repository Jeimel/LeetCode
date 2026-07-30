impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let n = word.len() as i32;
        let r = n / 8;

        (r + 1) * (-4 * r + n)
    }
}
